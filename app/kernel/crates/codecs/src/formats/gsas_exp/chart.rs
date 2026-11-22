use super::invalid;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use vesta_model::{Cell3, Result, source7::schema_validation};

pub(super) const CHART: &str = include_str!("../../../contracts/gsas-spin-chart.schema.json");
pub(super) const FRAME: &str = include_str!("../../../contracts/gsas-magnetic-frame.schema.json");
pub(super) const SPIN_RULE: &str = "draft.gsas-original-r1253-spin-chart";
pub(super) const MOMENT_RULE: &str = "draft.gsas-cms4-moment";
pub(super) const IDENTITY: [[i32; 3]; 3] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
const MAX_RATIONAL: i128 = (1 << 53) - 1;

fn validate<T: DeserializeOwned>(raw: &[u8], schema: &str) -> Result<T> {
    let value = vesta_model::json::parse(raw, 512 * 1024)?;
    let schema: Value = serde_json::from_str(schema).expect("validated frozen GSAS schema");
    schema_validation::validate(&value, &schema, "gsas", &Value::Null, 0)
        .map_err(|_| invalid("GSAS chart or frame violates its frozen schema"))?;
    serde_json::from_value(value).map_err(|_| invalid("GSAS chart or frame fields"))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(super) struct Rational {
    pub(super) numerator: i64,
    pub(super) denominator: i64,
}

impl Rational {
    const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    fn modulo(n: i128, d: i128) -> Result<Self> {
        let n = n.rem_euclid(d);
        let (mut a, mut b) = (n, d);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        let (n, d) = (n / a, d / a);
        if d > MAX_RATIONAL {
            return Err(invalid("GSAS canonical translation exceeds rational bound"));
        }
        Ok(Self {
            numerator: n as i64,
            denominator: d as i64,
        })
    }

    fn add_scaled(self, other: Self, multiplier: i32) -> Result<Self> {
        let (a, b, c, d) = (
            i128::from(self.numerator),
            i128::from(self.denominator),
            i128::from(other.numerator),
            i128::from(other.denominator),
        );
        Self::modulo(a * d + c * b * i128::from(multiplier), b * d)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Base {
    rotation: [[i32; 3]; 3],
    translation: [Rational; 3],
    generator_code_mask: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Chart {
    pub(super) source_input_sha256: String,
    pub(super) source_phase_number: u8,
    pub(super) source_group_symbol: String,
    pub(super) setting_name: String,
    pub(super) origin_choice: String,
    base_operators: Vec<Base>,
    centering_translations: Vec<[Rational; 3]>,
    generator_flags: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct Operation {
    pub(super) rotation: [[i32; 3]; 3],
    pub(super) translation: [Rational; 3],
}

impl Operation {
    const IDENTITY: Self = Self {
        rotation: IDENTITY,
        translation: [Rational::ZERO; 3],
    };

    pub(super) fn is_identity(&self) -> bool {
        *self == Self::IDENTITY
    }

    fn determinant(&self) -> i128 {
        let r = self.rotation.map(|row| row.map(i128::from));
        (0..3)
            .map(|i| {
                let j = (i + 1) % 3;
                let k = (i + 2) % 3;
                r[0][i] * (r[1][j] * r[2][k] - r[1][k] * r[2][j])
            })
            .sum()
    }

    fn compose(&self, rhs: &Self) -> Result<Self> {
        let mut rotation = [[0; 3]; 3];
        let mut translation = self.translation;
        for i in 0..3 {
            for (j, number) in rotation[i].iter_mut().enumerate() {
                let entry: i128 = (0..3)
                    .map(|k| i128::from(self.rotation[i][k]) * i128::from(rhs.rotation[k][j]))
                    .sum();
                *number = i32::try_from(entry)
                    .map_err(|_| invalid("GSAS operation rotation overflow"))?;
            }
            for k in 0..3 {
                translation[i] =
                    translation[i].add_scaled(rhs.translation[k], self.rotation[i][k])?;
            }
        }
        Ok(Self {
            rotation,
            translation,
        })
    }

    pub(super) fn wire(&self, id: &str, time_sign: i8) -> Value {
        json!({"id":id, "fractionalMatrix":self.rotation.map(|r| r.map(|n| Rational {
            numerator: i64::from(n), denominator: 1,
        })), "fractionalTranslation":self.translation, "timeSign":time_sign})
    }
}

impl Chart {
    pub(super) fn parse(raw: &[u8]) -> Result<Self> {
        validate(raw, CHART)
    }

    pub(super) fn expand(&self, signs: [i8; 12]) -> Result<Vec<(Operation, i8)>> {
        let generators = &self.generator_flags;
        let centers = &self.centering_translations;
        if generators.len() + centers.len() - 1 > signs.len()
            || generators.iter().collect::<BTreeSet<_>>().len() != generators.len()
            || centers[0].iter().any(|x| x.numerator != 0)
        {
            return Err(invalid(
                "GSAS spin slots or centering identity disagree with the chart",
            ));
        }
        let mut operations = Vec::new();
        for (i, center) in centers.iter().enumerate() {
            let centering_sign = if i == 0 {
                1
            } else {
                signs[generators.len() + i - 1]
            };
            for base in &self.base_operators {
                let mut translation = [Rational::ZERO; 3];
                for axis in 0..3 {
                    translation[axis] = translation[axis]
                        .add_scaled(base.translation[axis], 1)?
                        .add_scaled(center[axis], 1)?;
                }
                let operation = Operation {
                    rotation: base.rotation,
                    translation,
                };
                if !matches!(operation.determinant(), -1 | 1) {
                    return Err(invalid("GSAS rotation determinant must be ±1"));
                }
                let sign = generators
                    .iter()
                    .zip(signs)
                    .filter_map(|(flag, sign)| {
                        (base.generator_code_mask & *flag != 0).then_some(sign)
                    })
                    .product::<i8>();
                operations.push((operation, centering_sign * sign));
            }
        }
        let group: BTreeMap<_, _> = operations.iter().map(|(op, sign)| (*op, *sign)).collect();
        if group.len() != operations.len() || group.get(&Operation::IDENTITY) != Some(&1) {
            return Err(invalid(
                "GSAS group has duplicate operations or lacks positive identity",
            ));
        }
        for (left, a) in &operations {
            for (right, b) in &operations {
                let composition = left.compose(right)?;
                if group.get(&composition) != Some(&(*a * *b)) {
                    return Err(invalid(
                        "GSAS chart is not closed under its magnetic character",
                    ));
                }
            }
        }
        Ok(operations)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Frame {
    pub(super) scope_input_digest: String,
    source_cell_basis_a: [[f64; 3]; 3],
}

#[derive(Clone, Copy)]
pub(super) struct NumericalPolicy {
    pub(super) max_condition: f64,
    pub(super) length_a: f64,
    pub(super) angle_deg: f64,
    pub(super) orthonormal: f64,
}

pub(super) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[(i + 1) % 3] * b[(i + 2) % 3] - a[(i + 2) % 3] * b[(i + 1) % 3])
}

fn norm(v: [f64; 3]) -> f64 {
    v[0].hypot(v[1]).hypot(v[2])
}

impl Frame {
    pub(super) fn parse(raw: &[u8]) -> Result<Self> {
        validate(raw, FRAME)
    }

    pub(super) fn axes(
        &self,
        cell: &Cell3,
        policy: NumericalPolicy,
        operations: &[(Operation, i8)],
    ) -> Result<[[f64; 3]; 3]> {
        let [a, b, c]: [[f64; 3]; 3] =
            std::array::from_fn(|i| self.source_cell_basis_a.map(|row| row[i]));
        let adj = [cross(b, c), cross(c, a), cross(a, b)];
        let volume = dot(a, adj[0]);
        let lengths = [norm(a), norm(b), norm(c)];
        let condition = norm(lengths) * norm(adj.map(|axis| norm(axis) / volume.abs()));
        let close = |x: f64, y: f64, bound: f64| (x - y).abs() <= bound;
        if !volume.is_finite()
            || volume == 0.
            || !condition.is_finite()
            || condition > policy.max_condition
            || !lengths
                .into_iter()
                .zip(cell.length_a)
                .all(|(x, y)| close(x, y, policy.length_a))
        {
            return Err(invalid(
                "GSAS magnetic frame basis differs from its selected phase or policy",
            ));
        }
        let [x, v, w] = [a, b, c].map(|axis| axis.map(|n| n / norm(axis)));
        let angles =
            [dot(v, w), dot(x, w), dot(x, v)].map(|e| e.clamp(-1., 1.).acos().to_degrees());
        if !angles
            .into_iter()
            .zip(cell.angles_deg)
            .all(|(x, y)| close(x, y, policy.angle_deg))
        {
            return Err(invalid(
                "GSAS magnetic frame angles differ from the selected phase",
            ));
        }
        let c_star = adj[2].map(|n| n / volume);
        let z = c_star.map(|n| n / norm(c_star));
        let y = cross(z, x);
        let axes = [x, y, z];
        if !axes.iter().enumerate().all(|(i, &axis)| {
            axes.iter().enumerate().all(|(j, &other)| {
                close(
                    dot(axis, other),
                    if i == j { 1. } else { 0. },
                    policy.orthonormal,
                )
            })
        }) || !close(dot(cross(x, y), z), 1., policy.orthonormal)
        {
            return Err(invalid(
                "GSAS System 4 axes are not right-handed orthonormal",
            ));
        }
        for (op, _) in operations {
            let reciprocal = adj.map(|axis| axis.map(|n| n / volume));
            let transformed = self.source_cell_basis_a.map(|row| {
                let ar: [f64; 3] =
                    std::array::from_fn(|i| dot(row, op.rotation.map(|r| f64::from(r[i]))));
                std::array::from_fn::<_, 3, _>(|i| dot(ar, reciprocal.map(|r| r[i])))
            });
            if !transformed.iter().enumerate().all(|(i, &row)| {
                transformed.iter().enumerate().all(|(j, &other)| {
                    close(
                        dot(row, other),
                        if i == j { 1. } else { 0. },
                        policy.orthonormal,
                    )
                })
            }) {
                return Err(invalid(
                    "GSAS chart rotation does not preserve the qualified cell metric",
                ));
            }
        }
        Ok(axes)
    }
}
