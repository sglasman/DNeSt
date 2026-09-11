#[cfg(kani)]
use kani::Invariant;

use super::util;

#[derive(Copy, Clone)]
pub(super) struct Reader<'buf> {
    buf: &'buf [u8],
    pos: usize,
}

impl<'buf> Reader<'buf> {
    #[cfg_attr(kani, kani::ensures(|reader| reader.is_safe()))]
    pub(super) fn new(buf: &'buf [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub(super) fn get_current_pos(&self) -> usize {
        self.pos
    }

    pub(super) fn get_buf(&self) -> &'buf [u8] {
        self.buf
    }

    pub(super) fn is_safe(&self) -> bool {
        self.pos <= self.buf.len()
    }

    #[cfg_attr(
        kani,
        kani::modifies(&self.pos),
        kani::requires(self.is_safe()),
        kani::ensures(|result|
            self.is_safe() &&
            generic_advance_postcondition(N, &self, old(self.pos), &result)
        )
    )]
    pub(super) fn get_next_and_advance<const N: usize>(
        &mut self,
    ) -> Result<&'buf [u8; N], DecodeError> {
        self.get_next_and_advance_var(N)
            .map(|slice| slice.try_into().unwrap())
    }

    #[cfg_attr(
        kani,
        kani::modifies(&self.pos)
    )]
    pub(super) fn get_next_and_advance_var(&mut self, increment: usize) -> Result<&'buf [u8], DecodeError> {
        let old_pos = self.pos;
        self.advance_pos(increment)?;
        Ok(&self.buf[old_pos..self.pos])
    }

    #[cfg_attr(
        kani,
        kani::modifies(&self.pos),
        kani::requires(self.is_safe()),
        kani::ensures(|result|
            self.is_safe() &&
            generic_advance_postcondition(increment, &self, old(self.pos), &result)
        )
    )]
    fn advance_pos(&mut self, increment: usize) -> Result<(), DecodeError> {
        let next_pos = self
            .pos
            .checked_add(increment)
            .ok_or(DecodeError::ArithmeticOverflow)?;
        if next_pos > self.buf.len() {
            Err(DecodeError::OutOfBounds)
        } else {
            self.pos = next_pos;
            Ok(())
        }
    }
}

#[derive(PartialEq, Eq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub enum DecodeError {
    OutOfBounds,
    ArithmeticOverflow,
    InconsistentState,
    ForwardCompressionPointer,
    UnsupportedLabelType,
    NameLengthLimitExceeded,
}

#[cfg(kani)]
impl<'buf> Reader<'buf> {
    // Public accessors for Kani only.
    pub(super) fn get_pos(&self) -> &usize {
        // Return a pointer rather than the value so `kani::modifies` can use it.
        &self.pos
    }
}

#[cfg(kani)]
pub(super) fn with_arbitrary_reader<const N: usize>(
    content: impl for<'a> FnOnce(Reader<'a>),
) -> () {
    let bytes: [u8; N] = kani::any();
    let len: usize = kani::any();
    let pos: usize = kani::any();
    kani::assume(len <= N);
    kani::assume(pos <= len);
    let reader = Reader {
        buf: &bytes[..len],
        pos,
    };
    content(reader);
}

#[cfg(kani)]
pub(super) fn generic_advance_postcondition<T>(
    increment: usize,
    reader: &Reader,
    old_pos: usize,
    result: &Result<T, DecodeError>
) -> bool {
    match old_pos.checked_add(increment) {
        None => matches!(result, Err(DecodeError::ArithmeticOverflow)),
        Some(sum) => {
            if sum > reader.buf.len() {
                matches!(result, Err(DecodeError::OutOfBounds))
            } else {
                result.is_ok() && sum == reader.pos
            }
        }
    }
}

#[cfg(kani)]
pub(super) fn weak_advance_postcondition<T>(
    max_increment: usize,
    old_pos: usize,
    new_pos: usize,
    result: &Result<T, DecodeError>
) -> bool {
    if result.is_ok() {
        new_pos >= old_pos && new_pos - old_pos <= max_increment
    } else {
        true
    }
}

#[cfg(kani)]
#[kani::proof_for_contract(Reader::new)]
fn check_contract_reader_new() {
    const MAX_LEN: usize = 16;

    let bytes: [u8; 16] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= MAX_LEN);

    let reader = Reader::new(&bytes[..len]);
}

#[cfg(kani)]
#[kani::proof_for_contract(Reader::advance_pos)]
fn check_contract_advance_pos() {
    with_arbitrary_reader::<16>(|mut reader| {
        let increment = kani::any::<usize>();
        let _ = reader.advance_pos(increment);
    })
}