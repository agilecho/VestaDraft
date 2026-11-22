use super::{decode, encode};
use vesta_model::Result;

// Frozen pre-integration Draft implementation, used only as a test oracle.
#[path = "../tests/fixtures/base64_oracle.rs"]
mod oracle;

fn normalized(result: Result<Vec<u8>>) -> std::result::Result<Vec<u8>, (String, String)> {
    result.map_err(|error| (error.code.to_string(), error.message))
}

fn compare(text: &str) {
    assert_eq!(
        normalized(decode(text)),
        normalized(oracle::decode(text)),
        "input length {}, prefix {:?}",
        text.len(),
        text.get(..text.len().min(16))
    );
}

#[test]
fn draft_base64_matches_frozen_oracle() {
    for length in [0, 1, 2, 3, 4, 19, 65_534, 65_535, 65_536, 65_537, 65_538, 65_539] {
        let data: Vec<_> = (0..length).map(|index| (index as u8).wrapping_mul(37)).collect();
        let encoded = encode(&data);
        assert_eq!(encoded, oracle::encode(&data));
        compare(&encoded);
        if length <= 65_536 {
            assert_eq!(decode(&encoded).unwrap(), data);
        } else {
            assert!(decode(&encoded).is_err());
        }
    }
    for invalid in ["A/", "AB==", "AAB=", "AA_A", "A@==", "AA==AAAA", "AA==@@@@", "A A=", "AA\r\n", "é==", "é="] {
        compare(invalid);
    }
    let alphabet = b"AZaz09+/= -_@\r\n!BCE";
    for a in alphabet {
        for b in alphabet {
            for c in alphabet {
                for d in alphabet {
                    let bytes = [*a, *b, *c, *d];
                    compare(std::str::from_utf8(&bytes).unwrap());
                }
            }
        }
    }
}
