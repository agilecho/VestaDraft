use craft::mime::base64::STANDARD;
use vesta_model::{Error, Result};

const MAX_CHUNK_BYTES: usize = 65_536;
const MAX_ENCODED_BYTES: usize = MAX_CHUNK_BYTES.div_ceil(3) * 4;

pub fn encode(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}

pub fn decode(text: &str) -> Result<Vec<u8>> {
    if text.len() > MAX_ENCODED_BYTES || text.len() % 4 != 0 {
        return Err(Error::new("N_SCHEMA", "base64 chunk size/padding"));
    }
    // The protocol reports invalid alphabet before it reports non-canonical padding.
    if !text
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'='))
    {
        return Err(Error::new("N_SCHEMA", "RFC4648 alphabet"));
    }
    let invalid = || Error::new("N_SCHEMA", "canonical RFC4648 required");
    let bytes = STANDARD.decode(text).map_err(|_| invalid())?;
    if bytes.len() > MAX_CHUNK_BYTES {
        return Err(invalid());
    }
    Ok(bytes)
}

#[cfg(test)]
#[path = "base64_tests.rs"]
mod tests;
