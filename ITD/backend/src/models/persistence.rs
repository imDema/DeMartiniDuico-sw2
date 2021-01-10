use std::num::ParseIntError;

const K: u32 = 0xdeadbeef; // Note, for development purposes, should not be committed in version control
const N: u32 = 4;

fn round_fn(x: u16) -> u16 {
    let x = x as u32 ^ K;
    let k = x.wrapping_mul(10949).wrapping_add(1801) % 80833;
    ((k & 0xFFFF) ^ (k >> 16)) as u16
}


fn feistel(x: u32, n: u32) -> u32 {
    let mut l = (x & 0xFFFF) as u16;
    let mut r = (x >> 16) as u16;
    for _ in 0..n {
        let l1 = r;
        let r1 = l ^ round_fn(r);
        l = l1;
        r = r1;
    }
    (l as u32) << 16 | r as u32
}


///```rust
/// # use clup::models::persistence::{encode_serial, decode_serial};
/// let x = 1234;
/// let enc = encode_serial(x);
/// let dec = decode_serial(&enc);
/// assert_eq!(Ok(x), dec);
///```
pub fn encode_serial(id: i32) -> String {
    format!("{:x}", feistel(id as u32, N))
}

pub fn decode_serial(s: &str) -> Result<i32, ParseIntError> {
    let x = u32::from_str_radix(s, 16)?;
    Ok(feistel(x, N) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feistel_test() {
        let tests = (0..25000).chain(u32::MAX-25000..u32::MAX);
        for t in tests.into_iter() {
            let enc = feistel(t, N);
            assert_ne!(t, enc); // Not required, qualitative

            let dec = feistel(enc, N);
            assert_eq!(t, dec);
        }
    }

    #[test]
    fn encode_decode_test() {
        let tests = (0..25000).chain(i32::MAX-25000..i32::MAX);
        for t in tests.into_iter() {
            let enc = encode_serial(t);
            
            let dec = decode_serial(&enc);
            assert_eq!(Ok(t), dec);
        }
    }
}