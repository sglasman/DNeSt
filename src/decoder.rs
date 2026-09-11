use crate::reader::{DecodeError, Reader};

#[cfg(kani)]
use crate::reader::{generic_advance_postcondition, with_arbitrary_reader};

pub struct Decoder<'buf> {
    reader: Reader<'buf>,
}

impl<'buf> Decoder<'buf> {
    pub fn new(buf: &'buf [u8]) -> Self {
        Self {
            reader: Reader::new(buf),
        }
    }

    pub(super) fn get_current_pos(&self) -> usize {
        self.reader.get_current_pos()
    }

    pub(super) fn get_reader(&self) -> &Reader<'buf> {
        &self.reader
    }

    pub(super) fn get_next_and_advance_var(
        &mut self,
        increment: usize,
    ) -> Result<&'buf [u8], DecodeError> {
        self.reader.get_next_and_advance_var(increment)
    }

    pub(super) fn get_next_and_advance<const N: usize>(
        &mut self
    ) -> Result<&'buf [u8; N], DecodeError> {
        self.reader.get_next_and_advance::<N>()
    }
}

pub trait Decode<'a>: Sized {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self, DecodeError>;
}

impl<'a> Decode<'a> for u8 {
    #[cfg_attr(
        kani,
        kani::requires(decoder.is_safe()),
        kani::modifies(decoder.get_pos()),
        kani::ensures(|result| {
            decoder.is_safe() && decode_u8_postcondition(decoder, old(*decoder.get_pos()), result)
        })
    )]
    fn decode(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder.reader.get_next_and_advance::<1>().map(|arr| arr[0])
    }
}

impl<'a> Decode<'a> for u16 {
    #[cfg_attr(
        kani,
        kani::requires(decoder.is_safe()),
        kani::modifies(decoder.get_pos()),
        kani::ensures(|result| {
            decoder.is_safe() && decode_u16_postcondition(decoder, old(*decoder.get_pos()), result)
        })
    )]
    fn decode(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        decoder
            .reader
            .get_next_and_advance::<2>()
            .map(|arr| u16::from_be_bytes(*arr))
    }
}

#[cfg(kani)]
impl<'buf> Decoder<'buf> {
    // Public accessors for Kani only

    pub(super) fn get_pos(&self) -> &usize {
        self.reader.get_pos()
    }

    pub(super) fn get_buf(&self) -> &'buf [u8] {
        self.reader.get_buf()
    }

    pub(super) fn is_safe(&self) -> bool {
        self.reader.is_safe()
    }
}

#[cfg(kani)]
fn decode_u8_postcondition(
    decoder: &Decoder,
    old_pos: usize,
    result: &Result<u8, DecodeError>,
) -> bool {
    generic_advance_postcondition(1, &decoder.reader, old_pos, result)
        && if let Ok(byte) = result {
            *byte == decoder.get_buf()[old_pos]
        } else {
            true
        }
}

#[cfg(kani)]
fn decode_u16_postcondition(
    decoder: &Decoder,
    old_pos: usize,
    result: &Result<u16, DecodeError>,
) -> bool {
    generic_advance_postcondition(2, &decoder.reader, old_pos, result)
        && if let Ok(value) = result {
            *value
                == u16::from_be_bytes(
                    decoder.get_buf()[old_pos..*decoder.get_pos()]
                        .try_into()
                        .unwrap(),
                )
        } else {
            true
        }
}

#[cfg(kani)]
pub(crate) fn with_arbitrary_decoder<const N: usize>(
    content: impl for<'a> FnOnce(Decoder<'a>),
) -> () {
    with_arbitrary_reader::<N>(|reader| {
        content(Decoder { reader });
    })
}

#[cfg(kani)]
#[kani::proof_for_contract(u8::decode)]
#[kani::stub_verified(Reader::advance_pos)]
fn check_contract_u8_decode() {
    with_arbitrary_decoder::<8>(|mut decoder| {
        let _ = u8::decode(&mut decoder);
    })
}

#[cfg(kani)]
#[kani::proof_for_contract(u16::decode)]
#[kani::stub_verified(Reader::advance_pos)]
fn check_contract_u16_decode() {
    with_arbitrary_decoder::<8>(|mut decoder| {
        let _ = u16::decode(&mut decoder);
    })
}
