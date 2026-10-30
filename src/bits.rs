use std::{error, fmt};

pub struct BitWriter {
    output: Vec<u8>,
    current: u8,
    used: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        BitWriter { output: Vec::new(), current: 0, used: 0 }
    }

    pub fn write_bit(&mut self, bit: u8) {
        self.current |= bit << (7 - self.used);
        self.used += 1;
        if self.used == 8 {
            self.output.push(self.current);
            self.current = 0;
            self.used = 0;
        }
    }

    pub fn write_bits(&mut self, value: u32, count: u8) {
        for n in (0..count).rev() {
            self.write_bit(((value >> n) & 1) as u8);
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        if self.used > 0 {
            self.output.push(self.current);
        }
        self.output
    }
}

pub struct BitReader<'a> {
    input: &'a [u8],
    pos: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BitReaderError {
    End,
}

impl fmt::Display for BitReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitReaderError::End => write!(f, "stream ended"),
        }
    }
}

impl error::Error for BitReaderError {}

impl<'a> BitReader<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        BitReader { input, pos: 0 }
    }

    pub fn read_bit(&mut self) -> Result<u8, BitReaderError> {
        if self.pos >= self.input.len() * 8 {
            return Err(BitReaderError::End);
        }
        let byte = self.input[self.pos / 8];
        let bit = 7 - (self.pos % 8);
        self.pos += 1;
        Ok((byte >> bit) & 1)
    }

    pub fn read_bits(&mut self, count: u8) -> Result<u32, BitReaderError> {
        let mut value: u32 = 0;
        for _ in 0..count {
            let bit = self.read_bit()? as u32;
            value = (value << 1) | bit; // equivalent to value |= bit << (count - 1 - i) if _ was i
        }
        Ok(value)
    }
}