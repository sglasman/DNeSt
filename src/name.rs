use crate::decoder::{Decode, Decoder};
use crate::reader::{DecodeError, Reader};
use crate::util;

#[cfg(kani)]
use crate::{ decoder::with_arbitrary_decoder, reader::weak_advance_postcondition };

pub struct Name<'buf> {
    buf: &'buf [u8],
    start_pos: usize,
}

impl<'buf> Name<'buf> {
    // pub fn labels(&self) -> impl Iterator<Item = Result<&'a [u8], DecodeError>> {
    //     Labels { reader: self.reader }
    // }

    #[cfg_attr(
        kani,
        kani::modifies(decoder.get_pos()),
        kani::requires(decoder.is_safe() && remaining_capacity > 0),
        kani::ensures(|result| decoder.is_safe() && weak_advance_postcondition(
            remaining_capacity,
            old(*decoder.get_pos()),
            *decoder.get_pos(),
            result
        ))
    )]
    fn advance_through_name(
        decoder: &mut Decoder<'buf>,
        mut remaining_capacity: usize,
    ) -> Result<(), DecodeError> {
        loop {
            let next_byte = u8::decode(decoder)?;
            match next_byte >> 6 {
                0 => {
                    let label_length = (next_byte & 63) as usize;
                    if label_length == 0 {
                        return Ok(())
                    } else if remaining_capacity <= label_length + 1 {
                        return Err(DecodeError::NameLengthLimitExceeded)
                    } else {
                        let _ = decoder.get_next_and_advance_var(label_length)?;
                        remaining_capacity = remaining_capacity - label_length - 1;
                    }
                }
                3 => {
                    return if remaining_capacity < 2 {
                        Err(DecodeError::NameLengthLimitExceeded)
                    } else {
                        let _ = decoder.get_next_and_advance::<1>()?;
                        Ok(())
                    }
                }
                _ => return Err(DecodeError::UnsupportedLabelType),
            }
        }
    }

    pub(super) const MAX_LENGTH: usize = 255;
    pub(super) const MAX_LABELS: usize = 2;
}

impl<'buf> Decode<'buf> for Name<'buf> {
    #[cfg_attr(
        kani,
        kani::modifies(decoder.get_pos()),
        kani::requires(decoder.is_safe()),
        kani::ensures(|result| decoder.is_safe() && weak_advance_postcondition(
            Name::MAX_LENGTH,
            old(*decoder.get_pos()),
            *decoder.get_pos(),
            result
        ))
    )]
    fn decode(decoder: &mut Decoder<'buf>) -> Result<Self, DecodeError> {
        let start_pos = decoder.get_current_pos();
        Name::advance_through_name(decoder, Name::MAX_LENGTH)?;
        Ok(Name {
            buf: decoder.get_reader().get_buf(),
            start_pos,
        })
    }
}

#[cfg(kani)]
#[kani::proof_for_contract(Name::advance_through_name)]
#[kani::stub_verified(u8::decode)]
#[kani::stub_verified(Reader::advance_pos)]
#[kani::solver(cvc5)]
fn check_contract_advance_through_name() {
    with_arbitrary_decoder::<4>(|mut decoder| {
        let remaining_capacity: usize = kani::any::<usize>();
        kani::assume(remaining_capacity > 0);
        Name::advance_through_name(&mut decoder, remaining_capacity);
    })
}

#[cfg(kani)]
#[kani::proof_for_contract(Name::decode)]
#[kani::stub_verified(Name::advance_through_name)]
fn check_contract_name_decode() {
    with_arbitrary_decoder::<16>(|mut decoder| {
        let _ = Name::decode(& mut decoder);
    })
}


// struct Labels<'a> {
//     reader: &'a Reader<'a>
// }

//impl<'a> Labels<'a> {
// #[cfg_attr(kani, kani::modifies(&self.current_pos))]
// fn get_next_and_advance<const N: usize>(&mut self) -> Result<&[u8; N], DecodeError> {
//     util::get_next_and_advance::<N>(self.buf, &mut self.current_pos)
// }
//
// #[cfg_attr(kani, kani::modifies(&self.current_pos))]
// fn get_next_and_advance_var(&mut self, length: usize) -> Result<&'a [u8], DecodeError> {
//     util::get_next_and_advance_var(self.buf, &mut self.current_pos, length)
// }

//     fn read_next_label(&mut self, byte: u8) -> Option<Result<&'a [u8], DecodeError>> {
//         if byte == 0 { None } // Name terminated
//         else {
//             Some(self.reader.get_next_and_advance_var(byte as usize))
//         }
//     }
//
//     fn read_compression_pointer(&mut self, byte: u8) -> Option<Result<&'a [u8], DecodeError>> {
//         todo!()
//     }
// }

// impl<'a> Iterator for Labels<'a> {
//     type Item = Result<&'a [u8], DecodeError>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.reader.get_next_and_advance::<1>() {
//             Err(err) => Some(Err(err)),
//             Ok(bytes) => {
//                 let byte: u8 = bytes[0];
//                 match (byte >> 6) & 3 {
//                     0 => self.read_next_label(byte),
//                     3 => self.read_compression_pointer(byte),
//                     _ => Some(Err(DecodeError::UnsupportedLabelType))
//                 }
//             }
//         }
//     }
// }
