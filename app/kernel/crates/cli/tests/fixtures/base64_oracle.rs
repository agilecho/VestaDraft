use vesta_model::{Error, Result};
const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub fn encode(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len().div_ceil(3) * 4);
    for c in b.chunks(3) {
        let n = ((c[0] as u32) << 16)
            | ((c.get(1).copied().unwrap_or(0) as u32) << 8)
            | (c.get(2).copied().unwrap_or(0) as u32);
        s.push(A[((n >> 18) & 63) as usize] as char);
        s.push(A[((n >> 12) & 63) as usize] as char);
        s.push(if c.len() > 1 {
            A[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        s.push(if c.len() > 2 {
            A[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    s
}
pub fn decode(s: &str) -> Result<Vec<u8>> {
    if s.len() > 87384 || s.len() % 4 != 0 {
        return Err(Error::new("N_SCHEMA", "base64 chunk size/padding"));
    }
    let mut out = Vec::new();
    for c in s.as_bytes().chunks(4) {
        let mut n = 0u32;
        for b in c {
            n = (n << 6)
                | if *b == b'=' {
                    0
                } else {
                    A.iter()
                        .position(|v| v == b)
                        .ok_or_else(|| Error::new("N_SCHEMA", "RFC4648 alphabet"))?
                        as u32
                };
        }
        out.push((n >> 16) as u8);
        if c[2] != b'=' {
            out.push((n >> 8) as u8)
        }
        if c[3] != b'=' {
            out.push(n as u8)
        }
    }
    if out.len() > 65536 || encode(&out) != s {
        return Err(Error::new("N_SCHEMA", "canonical RFC4648 required"));
    }
    Ok(out)
}
