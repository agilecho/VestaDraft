use crate::{Error, Result};
vesta_source_derive::source_model! {pub struct Cell3 {pub length_a:[f64;3],pub angles_deg:[f64;3]}}
impl Cell3 {
    pub fn validate(&self) -> Result<()> {
        if self.length_a.iter().any(|x| !x.is_finite() || *x <= 0.)
            || self
                .angles_deg
                .iter()
                .any(|x| !x.is_finite() || *x <= 0. || *x >= 180.)
        {
            return Err(Error::new(
                "E_DOMAIN",
                "finite positive cell lengths and angles in (0,180) required",
            ));
        }
        let [a, b, c] = self.angles_deg.map(|a| a.to_radians().cos());
        let gram = 1. + 2. * a * b * c - a * a - b * b - c * c;
        if !gram.is_finite() || gram <= 1e-14 {
            return Err(Error::new(
                "E_DOMAIN",
                "singular or nonpositive cell metric",
            ));
        }
        Ok(())
    }
    pub fn volume(&self) -> Result<f64> {
        self.validate()?;
        let [a, b, c] = self.angles_deg.map(|a| a.to_radians().cos());
        let v = self.length_a.iter().product::<f64>()
            * (1. + 2. * a * b * c - a * a - b * b - c * c).sqrt();
        if !v.is_finite() || v <= 0. {
            return Err(Error::new("E_DOMAIN", "cell volume overflow"));
        }
        Ok(v)
    }
}
