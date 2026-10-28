pub mod bits;
use std::{error, fmt};

const MARKER: u8 = 0xFF;

#[derive(Debug, PartialEq, Eq)]
pub enum DecompressError {
    Truncated,
    ZeroCount,
}

impl fmt::Display for DecompressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecompressError::Truncated => write!(f, "stream truncated"),
            DecompressError::ZeroCount => write!(f, "run with zero count"),
        }
    }
}

impl error::Error for DecompressError {}

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

pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, DecompressError> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b != MARKER {
            out.push(b);
            i += 1;
        } else {
            if i + 2 < bytes.len() {
                let count = bytes[i + 1];
                if count == 0 {
                    return Err(DecompressError::ZeroCount)
                }
                let byte = bytes[i + 2];
                for _ in 0..count {
                    out.push(byte);
                }
                i += 3;
            } else {
                return Err(DecompressError::Truncated)
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::bits::{BitReader, BitReaderError, BitWriter};

use super::{compress, decompress};

    fn trip(original: &[u8]) {
        let compressed = compress(original);
        let restored = decompress(&compressed).unwrap();
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

    #[test]
    fn badtest() {
        trip(&[0xFF]);
    }

    #[test]
    fn bitwritertest() {
        let mut bw = BitWriter::new();
        bw.write_bits(1,1);
        bw.write_bits(0b01,2);
        bw.write_bits(0b1111,4);
        bw.write_bits(0,1);
        bw.write_bits(0b11,2);
        let output = bw.finish();
        assert_eq!(output, [0xBE, 0xC0]);
    }

    #[test]
    fn bitreaderwritertest() {
        let mut bw = BitWriter::new();
        bw.write_bits(0b101, 3);
        bw.write_bits(0b11, 2);
        let output = bw.finish();
        let mut br = BitReader::new(&output);
        assert_eq!(br.read_bits(3).unwrap(), 0b101);
        assert_eq!(br.read_bits(2).unwrap(), 0b11);
    }

    #[test]
    fn read_from_empty_fails() {
        let mut reader = BitReader::new(&[]);
        assert_eq!(reader.read_bit(), Err(BitReaderError::End));
    }

    #[test]
    fn read_past_end_fails() {
        // 1 byte = 8 bits, so asking for 9 must fail
        let mut reader = BitReader::new(&[0xFF]);
        assert_eq!(reader.read_bits(9), Err(BitReaderError::End));

        // exactly 8 bits is fine, but the next bit is past the end
        let mut reader = BitReader::new(&[0xFF]);
        assert_eq!(reader.read_bits(8), Ok(0xFF));
        assert_eq!(reader.read_bit(), Err(BitReaderError::End));
    }

}
