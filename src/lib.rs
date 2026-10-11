const MARKER: u8 = 0xFF;

pub fn compress(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let mut count = 1;
        while i + count < bytes.len() && bytes[i + count] == b && count < 255 {
            count += 1;
        }

        if b == MARKER || count >= 4 {
            out.push(MARKER);
            out.push(count as u8);
            out.push(b);
        } else {
            for _ in 0..count {
                out.push(b);
            }
        }

        i += count;
    }
    out
}

pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b != MARKER {
            out.push(b);
            i += 1;
        } else {
            let count = bytes[i+1];
            let byte = bytes[i+2];
            for _ in 0..count {
                out.push(byte);
            }
            i += 3;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{compress, decompress};

    fn trip(original: &[u8]) {
        let compressed = compress(original);
        let restored = decompress(&compressed);
        assert_eq!(restored, original);
    }

    #[test]
    fn empty_input() {
        trip(&[]);
    }

    #[test]
    fn one_byte() {
        trip(&[b'a']);
    }

    #[test]
    fn repeated_bytes() {
        trip(&[b'a'; 200]);
    }

    #[test]
    fn run_length_boundaries() {
        for len in [254, 255, 256, 257, 510, 511, 512, 100_000] {
            trip(&vec![b'a'; len]);
        }
    }

    #[test]
    fn all_byte_values() {
        let data: Vec<u8> = (0..=255).collect();
        trip(&data);
    }

    #[test]
    fn random_alphanumeric() {
        const CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let data: Vec<u8> = (0..1000)
            .map(|_| CHARS[rand::random_range(0..CHARS.len())])
            .collect();
        trip(&data);
    }

    #[test]
    fn random_bytes() {
        let data: Vec<u8> = (0..1000).map(|_| rand::random::<u8>()).collect();
        trip(&data);
    }
}