use std::collections::VecDeque;

use miden_processor::Felt as RawFelt;
use proptest::{
    arbitrary::Arbitrary,
    strategy::{BoxedStrategy, Strategy},
};

pub trait PushToStack: Sized {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        let mut ptr = self as *const Self as *const u8;
        let mut num_bytes = core::mem::size_of::<Self>();
        let mut buf = Vec::with_capacity(num_bytes / core::mem::size_of::<u32>());
        while num_bytes > 0 {
            let mut next = [0u8; 4];
            let consume = core::cmp::min(4, num_bytes);
            unsafe {
                ptr.copy_to_nonoverlapping(next.as_mut_ptr(), consume);
                ptr = ptr.byte_add(consume);
            }
            num_bytes -= consume;
            buf.push(RawFelt::new(u32::from_be_bytes(next) as u64));
        }

        for item in buf.into_iter().rev() {
            stack.push(item);
        }
    }
}

pub trait PopFromStack: Sized {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        use core::mem::MaybeUninit;

        let mut num_bytes = core::mem::size_of::<Self>();
        let mut result = MaybeUninit::<Self>::uninit();
        let mut ptr = result.as_mut_ptr() as *mut u8;
        while num_bytes > 0 {
            let next = stack.pop_front().expect("expected more operand stack elements");
            let next_bytes = (next.0.as_int() as u32).to_be_bytes();
            let consume = core::cmp::min(4, num_bytes);
            unsafe {
                next_bytes.as_ptr().copy_to_nonoverlapping(ptr, consume);
                ptr = ptr.byte_add(consume);
            }
            num_bytes -= consume;
        }
        Some(unsafe { result.assume_init() })
    }
}

impl PushToStack for bool {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u64))
    }
}
impl PopFromStack for bool {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() != 0)
    }
}

impl PushToStack for u8 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u64))
    }
}
impl PopFromStack for u8 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() as u8)
    }
}

impl PushToStack for i8 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u8 as u64))
    }
}
impl PopFromStack for i8 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() as i8)
    }
}

impl PushToStack for u16 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u64))
    }
}
impl PopFromStack for u16 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() as u16)
    }
}

impl PushToStack for i16 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u16 as u64))
    }
}
impl PopFromStack for i16 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() as i16)
    }
}

impl PushToStack for u32 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u64))
    }
}
impl PopFromStack for u32 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() as u32)
    }
}

impl PushToStack for i32 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(RawFelt::new(*self as u32 as u64))
    }
}
impl PopFromStack for i32 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front().unwrap().0.as_int() as i32)
    }
}

impl PushToStack for u64 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        let lo = self.rem_euclid(2u64.pow(32));
        let hi = self.div_euclid(2u64.pow(32));
        stack.push(RawFelt::new(lo));
        stack.push(RawFelt::new(hi));
    }
}
impl PopFromStack for u64 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        let hi = stack.pop_front().unwrap().0.as_int() * 2u64.pow(32);
        let lo = stack.pop_front().unwrap().0.as_int();
        Some(hi + lo)
    }
}

impl PushToStack for i64 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        (*self as u64).try_push(stack)
    }
}
impl PopFromStack for i64 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        u64::try_pop(stack).map(|value| value as i64)
    }
}

impl PushToStack for u128 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        let lo = self.rem_euclid(2u128.pow(64));
        let hi = self.div_euclid(2u128.pow(64));
        (lo as u64).try_push(stack);
        (hi as u64).try_push(stack);
    }
}
impl PopFromStack for u128 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        let hi = (u64::try_pop(stack).unwrap() as u128) * 2u128.pow(64);
        let lo = u64::try_pop(stack).unwrap() as u128;
        Some(hi + lo)
    }
}

impl PushToStack for i128 {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        (*self as u128).try_push(stack)
    }
}
impl PopFromStack for i128 {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        u128::try_pop(stack).map(|value| value as i128)
    }
}

impl PushToStack for RawFelt {
    #[inline(always)]
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(*self);
    }
}
impl PopFromStack for RawFelt {
    #[inline(always)]
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        Some(stack.pop_front()?.0)
    }
}

impl PushToStack for Felt {
    #[inline(always)]
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        stack.push(self.0);
    }
}
impl PopFromStack for Felt {
    #[inline(always)]
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        stack.pop_front()
    }
}

impl<const N: usize> PushToStack for [u8; N] {
    fn try_push(&self, stack: &mut Vec<RawFelt>) {
        let mut iter = self.iter().array_chunks::<4>();
        let buf_size = (self.len() / 4) + (self.len() % 4 == 0) as usize;
        let mut buf = vec![0u32; buf_size];
        let mut i = 0;
        for chunk in iter.by_ref() {
            let n = u32::from_be_bytes([*chunk[0], *chunk[1], *chunk[2], *chunk[3]]);
            buf[i] = n;
            i += 1;
        }
        if let Some(rest) = iter.into_remainder() {
            let mut n_buf = [0u8; 4];
            for (i, byte) in rest.into_iter().enumerate() {
                n_buf[i] = *byte;
            }
            buf[i] = u32::from_be_bytes(n_buf);
        }
        for chunk in buf.into_iter().rev() {
            PushToStack::try_push(&chunk, stack);
        }
    }
}

impl<const N: usize> PopFromStack for [u8; N] {
    fn try_pop(stack: &mut VecDeque<Felt>) -> Option<Self> {
        let mut out = [0u8; N];

        let byte_size = out.len();
        let mut i = 0;
        while i < byte_size {
            let chunk: u32 = PopFromStack::try_pop(stack).expect("invalid u32");
            let bytes = chunk.to_be_bytes();
            if i + 4 > byte_size {
                for byte in bytes[..(byte_size - i)].iter().copied() {
                    out[i] = byte;
                    i += 1;
                }
                break;
            } else {
                for byte in bytes.iter().copied() {
                    out[i] = byte;
                    i += 1;
                }
            }
        }

        Some(out)
    }
}

/// Wrapper around `miden_processor::Felt` that implements useful traits that are not implemented
/// for that type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Felt(pub RawFelt);

impl clap::builder::ValueParserFactory for Felt {
    type Parser = FeltParser;

    fn value_parser() -> Self::Parser {
        FeltParser
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct FeltParser;
impl clap::builder::TypedValueParser for FeltParser {
    type Value = Felt;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::error::Error> {
        use clap::error::{Error, ErrorKind};

        let value = value.to_str().ok_or_else(|| Error::new(ErrorKind::InvalidUtf8))?;

        let value = value.parse::<u64>().map_err(|err| {
            Error::raw(ErrorKind::InvalidValue, format!("invalid field element value: {err}"))
        })?;

        RawFelt::try_from(value)
            .map(Felt)
            .map_err(|err| Error::raw(ErrorKind::InvalidValue, err))
    }
}

impl From<Felt> for miden_processor::Felt {
    fn from(f: Felt) -> Self {
        f.0
    }
}

impl From<bool> for Felt {
    fn from(b: bool) -> Self {
        Self(RawFelt::from(b as u32))
    }
}

impl From<u8> for Felt {
    fn from(t: u8) -> Self {
        Self(t.into())
    }
}

impl From<i8> for Felt {
    fn from(t: i8) -> Self {
        Self((t as u8).into())
    }
}

impl From<i16> for Felt {
    fn from(t: i16) -> Self {
        Self((t as u16).into())
    }
}

impl From<u16> for Felt {
    fn from(t: u16) -> Self {
        Self(t.into())
    }
}

impl From<i32> for Felt {
    fn from(t: i32) -> Self {
        Self((t as u32).into())
    }
}

impl From<u32> for Felt {
    fn from(t: u32) -> Self {
        Self(t.into())
    }
}

impl From<u64> for Felt {
    fn from(t: u64) -> Self {
        Self(RawFelt::new(t))
    }
}

impl From<i64> for Felt {
    fn from(t: i64) -> Self {
        Self(RawFelt::new(t as u64))
    }
}

// Reverse Felt to Rust types conversion

impl From<Felt> for bool {
    fn from(f: Felt) -> Self {
        f.0.as_int() != 0
    }
}

impl From<Felt> for u8 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as u8
    }
}

impl From<Felt> for i8 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as i8
    }
}

impl From<Felt> for u16 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as u16
    }
}

impl From<Felt> for i16 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as i16
    }
}

impl From<Felt> for u32 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as u32
    }
}

impl From<Felt> for i32 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as i32
    }
}

impl From<Felt> for u64 {
    fn from(f: Felt) -> Self {
        f.0.as_int()
    }
}

impl From<Felt> for i64 {
    fn from(f: Felt) -> Self {
        f.0.as_int() as i64
    }
}

impl Arbitrary for Felt {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use miden_core::StarkField;
        (0u64..RawFelt::MODULUS).prop_map(|v| Felt(RawFelt::new(v))).boxed()
    }
}
