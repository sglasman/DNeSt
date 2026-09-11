use crate::reader::DecodeError;

pub fn bit_of(value: u16, i: usize) -> bool {
    (value >> (15 - i)) & 1 == 1
}

