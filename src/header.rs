use crate::reader::{DecodeError};
use crate::util::bit_of;

#[cfg(kani)]
use {
    crate::reader::generic_advance_postcondition, crate::decoder::with_arbitrary_decoder,
    kani::Invariant,
};
use crate::decoder::{Decode, Decoder};

#[derive(PartialEq, Eq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Header {
    id: u16,
    flags: Flags,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl<'a> Decode<'a> for Header {
    #[cfg_attr(kani,
        kani::requires(decoder.is_safe()),
        kani::modifies(decoder.get_pos()),
        kani::ensures(|result|
            decoder.is_safe() &&
            generic_advance_postcondition(
                12,
                decoder.get_reader(),
                old(*decoder.get_pos()),
                result
            )
        )
    )]
    fn decode(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let id = u16::decode(decoder)?;
        let flags = Flags::decode(decoder)?;
        let qdcount = u16::decode(decoder)?;
        let ancount = u16::decode(decoder)?;
        let nscount = u16::decode(decoder)?;
        let arcount = u16::decode(decoder)?;
        Ok(Self {
            id,
            flags,
            qdcount,
            ancount,
            nscount,
            arcount,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Flags {
    kind: MessageKind,
    opcode: Opcode,
    authoritative_answer: bool,
    truncated: bool,
    recursion_desired: bool,
    recursion_available: bool,
    z: bool,
    authentic_data: bool,
    checking_disabled: bool,
    rcode: RCode,
}

impl<'a> Decode<'a> for Flags {
    #[cfg_attr(kani,
        kani::requires(decoder.is_safe()),
        kani::modifies(decoder.get_pos()),
        kani::ensures(|result|
            decoder.is_safe() &&
            generic_advance_postcondition(
                2,
                decoder.get_reader(),
                old(*decoder.get_pos()),
                result
            )
        )
    )]
    fn decode(decoder: &mut Decoder) -> Result<Self, DecodeError> {
        let bytes = u16::decode(decoder)?;
        let kind = if bit_of(bytes, 0) {
            MessageKind::Reply
        } else {
            MessageKind::Query
        };
        let opcode =
            Opcode::new(((bytes >> 11) & 15) as u8).ok_or(DecodeError::InconsistentState)?;
        let rcode = RCode::new((bytes & 15) as u8).ok_or(DecodeError::InconsistentState)?;
        Ok(Flags {
            kind,
            opcode,
            authoritative_answer: bit_of(bytes, 5),
            truncated: bit_of(bytes, 6),
            recursion_desired: bit_of(bytes, 7),
            recursion_available: bit_of(bytes, 8),
            z: bit_of(bytes, 9),
            authentic_data: bit_of(bytes, 10),
            checking_disabled: bit_of(bytes, 11),
            rcode,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub enum MessageKind {
    Query,
    Reply,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Opcode(u8);

impl Opcode {
    pub const QUERY: Opcode = Opcode(0);
    pub const INVERSE_QUERY: Opcode = Opcode(1);
    pub const STATUS: Opcode = Opcode(2);
    pub fn new(x: u8) -> Option<Self> {
        if x == x & 15 { Some(Opcode(x)) } else { None }
    }
}

#[cfg(kani)]
impl kani::Arbitrary for Opcode {
    fn any() -> Self {
        let x = kani::any::<u8>();
        kani::assume(x == x & 15);
        Opcode::new(x).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RCode(u8);

impl RCode {
    pub const NO_ERROR: RCode = RCode(0);
    pub const FORM_ERROR: RCode = RCode(1);
    pub const SERVER_FAILURE: RCode = RCode(2);
    pub const NONEXISTENT_DOMAIN: RCode = RCode(3);
    pub const NOT_IMPLEMENTED: RCode = RCode(4);
    pub const QUERY_REFUSED: RCode = RCode(5);
    pub const NAME_EXISTS_WHEN_IT_SHOULD_NOT: RCode = RCode(6);
    pub fn new(x: u8) -> Option<Self> {
        if x == x & 15 { Some(RCode(x)) } else { None }
    }
}

#[cfg(kani)]
impl kani::Arbitrary for RCode {
    fn any() -> Self {
        let x = kani::any::<u8>();
        kani::assume(x == x & 15);
        RCode::new(x).unwrap()
    }
}

#[cfg(kani)]
#[kani::proof_for_contract(Flags::decode)]
#[kani::stub_verified(u16::decode)]
fn check_contract_flags_decode() {
    with_arbitrary_decoder::<8>(|mut decoder| {
        let _ = Flags::decode(&mut decoder);
    })
}

#[cfg(kani)]
#[kani::proof_for_contract(Header::decode)]
#[kani::stub_verified(u16::decode)]
#[kani::stub_verified(Flags::decode)]
fn check_contract_header_decode() {
    with_arbitrary_decoder::<16>(|mut decoder| {
        let _ = Header::decode(&mut decoder);
    })
}
