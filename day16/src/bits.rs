pub struct BitReader {
    bytes: Vec<u8>,
    byte_offset: usize,
    remaining_bits: usize,
    current_byte: u8,
}

impl BitReader {
    pub fn new(input: &str) -> Self {
        let bytes = str_to_bytes(input);
        let first_byte = bytes[0];

        BitReader {
            bytes,
            byte_offset: 0,
            remaining_bits: 8,
            current_byte: first_byte,
        }
    }

    pub fn read(&mut self, nbits: usize) -> Option<u8> {
        if nbits > 8 {
            panic!("can read 8 bits max")
        }

        if self.byte_offset >= self.bytes.len() {
            return None;
        }

        if nbits <= self.remaining_bits {
            let result = self.read_from_current_byte(nbits);
            self.remaining_bits -= nbits;

            if self.remaining_bits != 0 {
                self.current_byte = self.current_byte << nbits;
            }

            self.update_state();

            return Some(result);
        } else {
            let remainder_bits = nbits - self.remaining_bits;

            let upper = self.read_from_current_byte(self.remaining_bits);
            self.remaining_bits = 0;
            self.update_state();

            let lower = self.read_from_current_byte(remainder_bits);
            self.remaining_bits = 8 - remainder_bits;

            let result = (upper << remainder_bits) | lower;

            self.current_byte = self.current_byte << remainder_bits;

            return Some(result);
        }
    }

    pub fn read_u64(&mut self, nbits: usize) -> Option<u64> {
        let nbytes = (nbits / 8) + 1;
        let trailing_bits = nbits % 8;

        let mut result: u64 = 0;

        for i in 0..nbytes {
            let bits = if i == nbytes - 1 {
                trailing_bits
            } else {
                8
            };

            let value = self.read(bits)? as u64;

            let byte_index = nbytes - 1 - i;

            if byte_index == 0 {
                result = result | value;
            } else {
                result = result | (value << (trailing_bits + (8 * (byte_index - 1))));
            }
        }


        Some(result)
    }

    fn read_from_current_byte(&self, nbits: usize) -> u8 {
        if nbits == 8 {
            self.current_byte
        } else {
            ((self.current_byte >> (8 - nbits))) & (2_u8.pow(nbits as u32) - 1)
        }
    }

    fn update_state(&mut self) {
        if self.remaining_bits == 0 {
            self.remaining_bits = 8;

            if self.byte_offset + 1 < self.bytes.len() {
                self.byte_offset += 1;
                self.current_byte = self.bytes[self.byte_offset];
            }
        }
    }
}

pub fn print_binary(input: &str) {
    let bytes = str_to_bytes(input);

    for b in bytes {
        print!("{:b}", b);
    }

    println!();
}

fn str_to_bytes(input: &str) -> Vec<u8> {
    let input = if input.len() % 2 == 0 {
        input.to_string()
    } else {
        format!("{}0", input)
    };

    let bytes = input.chars().collect::<Vec<char>>()
        .chunks(2).into_iter()
        .map(|hex| format!("{}{}", hex[0], hex[1]))
        .map(|byte| u8::from_str_radix(&byte, 16).unwrap()).collect::<Vec<u8>>();

    bytes
}