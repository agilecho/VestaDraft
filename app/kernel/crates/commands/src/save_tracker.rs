//! Capturing candidate bytes is distinct from authenticated host settlement.
use std::collections::BTreeMap;
use vesta_model::*;
#[derive(Default)]
pub struct SaveTracker {
    pub pending: BTreeMap<String, Capture>,
    leases: BTreeMap<String, (String, String, u64)>,
}
impl SaveTracker {
    pub fn record(
        &mut self,
        id: &str,
        capture: Capture,
        owner: &str,
        token: &str,
        expires: u64,
    ) -> Result<()> {
        capture.content.validate()?;
        if owner.is_empty()
            || token.is_empty()
            || expires == 0
            || self.pending.contains_key(id)
            || self.pending.len() >= 16
        {
            return Err(Error::new(
                "E_RESOURCE_LIMIT",
                "snapshot owner/token/population",
            ));
        }
        self.pending.insert(id.into(), capture);
        self.leases
            .insert(id.into(), (owner.into(), token.into(), expires));
        Ok(())
    }
    pub fn confirm_untrusted(
        &mut self,
        id: &str,
        owner: &str,
        token: &str,
        now: u64,
    ) -> Result<()> {
        let (o, t, until) = self
            .leases
            .get(id)
            .ok_or_else(|| Error::new("E_TARGET_TOKEN_INVALID", "snapshot absent"))?;
        if now >= *until {
            return Err(Error::new(
                "E_TARGET_TOKEN_EXPIRED",
                "snapshot lease expired",
            ));
        }
        if o != owner || t != token {
            return Err(Error::new(
                "E_TARGET_TOKEN_INVALID",
                "snapshot authority/owner mismatch",
            ));
        }
        Err(Error::new(
            "E_TARGET_TOKEN_INVALID",
            "typed bytes are not private host/HS4 publication and semantic proof",
        ))
    }
    pub fn resident_bytes(&self) -> usize {
        self.pending
            .values()
            .map(|c| {
                c.resources.values().map(Vec::len).sum::<usize>()
                    + serde_json::to_vec(&c.content)
                        .map(|b| b.len())
                        .unwrap_or(16777216)
            })
            .sum()
    }
    pub fn count(&self) -> usize {
        self.pending.len()
    }
}
