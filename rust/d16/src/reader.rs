use std::cmp::min;
use std::io::BufRead;
use crate::packet::BitReader;

pub struct HexReader {
    hex: String,
    head: u32,
}

impl BitReader for HexReader {
    fn read(&mut self, n: u32) -> Option<u32> {
        if self.head + n > (self.hex.len() as u32) * 4 {
            None
        } else {
            let mut output = 0;
            let mut num_bits_to_read = n;

            while num_bits_to_read > 0 {
                let hex_head = (self.head / 8) as usize * 2;
                let digit_offset = self.head % 8;
                let num_bits_this_digit = min(8 - digit_offset, num_bits_to_read);
                let full_digit = u32::from_str_radix(&self.hex[hex_head..hex_head + 2], 16).unwrap();
                let mask = (1 << num_bits_this_digit) - 1;
                let shift = 8 - digit_offset - num_bits_this_digit;

                num_bits_to_read -= num_bits_this_digit;
                output |= (mask << shift & full_digit) >> shift << num_bits_to_read;
                self.head += num_bits_this_digit;
            }

            Some(output)
        }
    }
}

impl HexReader {
    pub fn head(&self) -> u32 {
        self.head
    }
}

impl<R: BufRead> From<R> for HexReader {
    fn from(reader: R) -> Self {
        Self {
            hex: reader.lines().next().unwrap().unwrap(),
            head: 0,
        }
    }
}
