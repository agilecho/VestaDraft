//! P2-03 active numeric model is three-dimensional. Higher-dimensional data is never flattened.
use crate::{Error, Result};
pub fn require_periodic_three(dimension: u8) -> Result<()> {
    if dimension == 3 {
        Ok(())
    } else {
        Err(Error::new(
            "E_NOT_IMPLEMENTED",
            "native dimension requires a later qualified domain implementation",
        ))
    }
}
