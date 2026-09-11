use crate::decoder::{Decode, Decoder};
use crate::name::Name;
use crate::reader::DecodeError;

pub struct Question<'buf> {
    name: Name<'buf>,
    qtype: u16,
    qclass: u16,
}

impl<'buf> Decode<'buf> for Question<'buf> {
    #[cfg_attr(kani,
        kani::modifies(decoder.get_pos()),
        kani::requires(decoder.is_safe()),
        kani::ensures(|result| decoder.is_safe() &&
            true //weak_advance_pos
        ),
        kani::stub_verified(Name::decode),
        kani::stub_verified
    )]
    fn decode(decoder: &mut Decoder<'buf>) -> Result<Self, DecodeError> {
        let name = Name::decode(decoder)?;
        let qtype = u16::decode(decoder)?;
        let qclass = u16::decode(decoder)?;
        Ok(Question {
            name,
            qtype,
            qclass,
        })
    }
}
