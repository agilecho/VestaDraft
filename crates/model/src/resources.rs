use crate::{Error, Result, digest};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug)]
struct Blob {
    bytes: Vec<u8>,
    owners: BTreeSet<String>,
}
#[derive(Clone, Debug, Default)]
pub struct ResourceStore {
    blobs: BTreeMap<String, Blob>,
}
impl ResourceStore {
    pub fn bytes(&self) -> usize {
        self.blobs.values().map(|v| v.bytes.len()).sum()
    }
    pub fn put(&mut self, bytes: Vec<u8>, owner: &str, limit: usize) -> Result<String> {
        if owner.is_empty() {
            return Err(Error::new("E_RESOURCE_MISSING", "empty resource owner"));
        }
        let id = digest(&bytes);
        if !self.blobs.contains_key(&id)
            && self
                .bytes()
                .checked_add(bytes.len())
                .is_none_or(|n| n > limit)
        {
            return Err(Error::new("E_RESOURCE_LIMIT", "resident resource budget"));
        }
        let b = self.blobs.entry(id.clone()).or_insert_with(|| Blob {
            bytes,
            owners: BTreeSet::new(),
        });
        b.owners.insert(owner.into());
        Ok(id)
    }
    pub fn read(&self, id: &str, owner: &str) -> Result<&[u8]> {
        self.blobs
            .get(id)
            .filter(|b| b.owners.contains(owner))
            .map(|b| b.bytes.as_slice())
            .ok_or_else(|| Error::new("E_RESOURCE_MISSING", "resource not retained by this owner"))
    }
    pub fn retain(&mut self, id: &str, from: &str, to: &str) -> Result<()> {
        self.read(id, from)?;
        if to.is_empty() {
            return Err(Error::new("E_RESOURCE_MISSING", "owner"));
        }
        self.blobs
            .get_mut(id)
            .ok_or_else(|| Error::new("E_RESOURCE_MISSING", "blob"))?
            .owners
            .insert(to.into());
        Ok(())
    }
    pub fn release(&mut self, id: &str, owner: &str) -> Result<bool> {
        let b = self
            .blobs
            .get_mut(id)
            .ok_or_else(|| Error::new("E_RESOURCE_MISSING", "blob"))?;
        if !b.owners.remove(owner) {
            return Err(Error::new("E_RESOURCE_MISSING", "owner lease"));
        }
        if b.owners.is_empty() {
            self.blobs.remove(id);
        }
        Ok(true)
    }
}
