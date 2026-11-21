//! Selected ORFFE result rows bound to an actual current P1 phase.
use crate::{Error, ImageKey, Result, source_audit as sa};
use serde_json::Value;
use vesta_source_derive::source_model;

source_model! {
    pub struct R80GeometryValue {
        pub value_lexeme: String,
        pub su_lexeme: String,
        pub native_value: f32,
        pub native_su: f32,
    }
}

pub const ACTIVE_TABLE_TAG: &str = "R80GeometryTable1";
pub const RIETAN_OWNER_DIALECT: &str = "native-rietan-phase-1";
pub const RIETAN_TABLE_DIALECT: &str = "native-rietan-orffe-1";

pub fn active_tables(source: &Value) -> Result<Vec<R80GeometryTable>> {
    let mut results = Vec::new();
    for row in source["metadata"].as_array().into_iter().flatten() {
        if row["tree"]["tag"] != ACTIVE_TABLE_TAG {
            continue;
        }
        let invalid = || Error::new("E_REOPEN_OWNER", "ORFFE table metadata owner or producer");
        let node = sa::Node::from_wire(&row["tree"]["payload"])?;
        let value = node.data()?;
        let table: R80GeometryTable =
            serde_json::from_value(value).map_err(|e| Error::new("E_SCHEMA", e.to_string()))?;
        if row["owner"]["kind"] != "phase"
            || row["owner"]["id"] != table.owner.as_str()
            || row["producerId"] != "VestaDraft"
            || !matches!(row["dialect"].as_str(), Some("native-r80-orffe-1" | RIETAN_TABLE_DIALECT))
            || results
                .iter()
                .any(|previous: &R80GeometryTable| previous.owner == table.owner)
        {
            return Err(invalid());
        }
        results.push(table);
    }
    Ok(results)
}

source_model! {
    /// Distance uses angstroms. Angle uses degrees and its middle image is the vertex.
    #[source(tag = "kind")]
    pub enum R80GeometryMeasurement {
        Distance { serial: u64, images: [ImageKey; 2], result: R80GeometryValue },
        Angle { serial: u64, images: [ImageKey; 3], result: R80GeometryValue },
    }
}

source_model! {
    pub struct R80GeometryTable {
        #[source(reference(Phase))]
        pub owner: sa::Id<sa::Phase>,
        #[source(reference(Resource))]
        pub resource: sa::Id<sa::Resource>,
        pub primary_sha256: String,
        pub table_sha256: String,
        pub rows: Vec<R80GeometryMeasurement>,
    }
}
