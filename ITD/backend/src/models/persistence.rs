use std::num::ParseIntError;

const K: u32 = 0xdeadbeef; // TODO: Use environment variable instead of constant
const N: u32 = 32;


fn round_fn(x: u16, k: u32) -> u16 {
    // Simple key transformation
    let x = x as u32;
    let x0 = x ^ k.rotate_right(x >> 16-5);
    // LCG
    let x1 = x0
        .wrapping_mul(2697822563)
        .wrapping_add(4212697711);

    // Permutation inspired from https://www.pcg-random.org/pdf/hmc-cs-2014-0905.pdf
    const ROTATE: u16 = 32-4;
    const XSHIFT: u16 = (16+4) / 2;
    const SPARE: u16 = 16-4;

    let rot = (x1 >> ROTATE) as u32;
    let xsh = (((x1 >> XSHIFT) ^ x1) >> SPARE) as u16;
    xsh.rotate_right(rot)
}

fn feistel(x: u32, n: u32, k: u32) -> u32 {
    let mut l = (x >> 16) as u16;
    let mut r = (x & 0xFFFF) as u16;
    for _ in 0..n {
        let l1 = r;
        let r1 = l ^ round_fn(r, k);
        l = l1;
        r = r1;
    }
    (r as u32) << 16 | l as u32
}

/// Pseudo encryption for serials, this **must not** be considered cryptographically secure since it has not been audited,
/// Only use for non critical data.
/// Intended for encoding serials to use in URLs and requests
/// ```rust
/// # use clup::models::persistence::{encode_serial, decode_serial};
/// let x = 1234;
/// let enc = encode_serial(x);
/// let dec = decode_serial(&enc);
/// assert_eq!(Ok(x), dec);
///```
pub fn encode_serial(id: i32) -> String {
    format!("{:x}", feistel(id as u32, N, K))
}

/// Decode integers encoded with encode_serial
pub fn decode_serial(s: &str) -> Result<i32, ParseIntError> {
    let x = u32::from_str_radix(s, 16)?;
    Ok(feistel(x, N, K) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    const K: u32 = 0xaddadada;

    #[test]
    fn feistel_test() {
        let tests = (0..25000).chain(u32::MAX-25000..u32::MAX);
        for t in tests.into_iter() {
            let enc = feistel(t, N, K);

            let dec = feistel(enc, N, K);
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
    
    // Pseudo encryption quality testing
    use image::{ImageBuffer, Rgb};

    const TEST_DIR: &'static str = "/tmp";
    const V: f32 = 1.0;
    const S: f32 = 1.0;
    fn num_to_col(num: u32) -> Rgb<u8> {
        let c = V * S;
        
        let h = num as f32 / u32::MAX as f32 * 6.;
        let x = c * (1. - (h % 2. - 1.).abs());

        let (r, g, b) = 
             if h < 1. {(c,x,0.)}
        else if h < 2. {(x,c,0.)}
        else if h < 3. {(0.,c,x)}
        else if h < 4. {(0.,x,c)}
        else if h < 5. {(x,0.,c)}
        else if h < 6.0005 {(c,0.,x)}
        else {dbg!(h); unreachable!()};

        let m = V - c;
        let r = ((r + m) * 255.) as u8;
        let g = ((g + m) * 255.) as u8;
        let b = ((b + m) * 255.) as u8;

        Rgb([r,g,b])
    }
    #[test]
    #[ignore]
    fn feistel_plot() {
        let (w, h) = (256, 256);
        let scale = 2;
        let tests = 0..w*h;
        let first = 0..=16u32;
        let large = [32,64].iter().map(|x|*x);
        for n in first.chain(large.into_iter()) {
            let cols: Vec<Rgb<u8>> = tests.clone()
                .map(|t| feistel(t, n, K))
                .map(num_to_col)
                .collect();
            let img = ImageBuffer::from_fn(w*scale, h*scale, |x,y| cols[(x/scale + y/scale*w) as usize]);
            img.save(format!("{}/feist_{}.png", TEST_DIR, n)).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn feistel_confusion() {
        const K_SIZE: usize = 32;
        const SPACE: u32 = 1024*1024;
        for flip in 0..K_SIZE {
            let k1 = K ^ (1 << flip);
            let mut counts = [0; K_SIZE];
            for x in 0..SPACE {
                let mut diff = feistel(x, N, K) ^ feistel(x, N, k1);
                for i in 0..K_SIZE {
                    counts[i] += diff & 1;
                    diff >>= 1;
                }
            }
            let probs: String = counts.iter()
                .map(|&cnt| (cnt as f32) / (SPACE as f32))
                .map(|p| format!("{:.3} ", p))
                .collect();
            println!("{:02}: {}", flip, probs);
        }
    }
    #[test]
    #[ignore]
    fn feistel_diffusion() {
        const K_SIZE: usize = 32;
        const SPACE: u32 = 1024*1024;
        for flip in 0..K_SIZE {
            let mut counts = [0; K_SIZE];
            for x in 0..SPACE {
                let x1 = x ^ (1 << flip);
                let mut diff = feistel(x, N, K) ^ feistel(x1, N, K);
                for i in 0..K_SIZE {
                    counts[i] += diff & 1;
                    diff >>= 1;
                }
            }
            let probs: String = counts.iter()
                .map(|&cnt| (cnt as f32) / (SPACE as f32))
                .map(|p| format!("{:.3} ", p))
                .collect();
            println!("{:02}: {}", flip, probs);
        }
    }
}