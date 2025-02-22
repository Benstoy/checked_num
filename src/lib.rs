#![no_std]

use core::num::NonZero;

use checked_num::CheckedNum;

mod builtin_int;
mod checked_num;

// `CheckedNum` is not publicly exposed to make
// changing bounds a backwards-compatible change.

pub type CheckedU128 = CheckedNum<u128>;
pub type CheckedU64 = CheckedNum<u64>;
pub type CheckedU32 = CheckedNum<u32>;
pub type CheckedU16 = CheckedNum<u16>;
pub type CheckedU8 = CheckedNum<u8>;

pub type CheckedI128 = CheckedNum<i128>;
pub type CheckedI64 = CheckedNum<i64>;
pub type CheckedI32 = CheckedNum<i32>;
pub type CheckedI16 = CheckedNum<i16>;
pub type CheckedI8 = CheckedNum<i8>;

pub type CheckedNonZeroU128 = CheckedNum<NonZero<u128>>;
pub type CheckedNonZeroU64 = CheckedNum<NonZero<u64>>;
pub type CheckedNonZeroU32 = CheckedNum<NonZero<u32>>;
pub type CheckedNonZeroU16 = CheckedNum<NonZero<u16>>;
pub type CheckedNonZeroU8 = CheckedNum<NonZero<u8>>;

pub type CheckedNonZeroI128 = CheckedNum<NonZero<i128>>;
pub type CheckedNonZeroI64 = CheckedNum<NonZero<i64>>;
pub type CheckedNonZeroI32 = CheckedNum<NonZero<i32>>;
pub type CheckedNonZeroI16 = CheckedNum<NonZero<i16>>;
pub type CheckedNonZeroI8 = CheckedNum<NonZero<i8>>;

#[test]
fn normal_add() {
    let a = CheckedU16::new(123);
    let b = 234;

    assert_eq!(a + b, 123 + b)
}

#[test]
fn overflowing_ne() {
    let a = CheckedU8::new(u8::MAX);
    let b = 1;

    // overflowed values should not be equal
    assert_ne!(a + b, a + b)
}

#[test]
fn underflow() {
    let a = CheckedI8::new(i8::MIN);
    let b = 1;

    assert!((a - b).did_overflow());
}

#[test]
fn bit_or() {
    let a = CheckedU8::new(0b11000011);

    let b_raw = 0b11110011;
    let b_wrapped = CheckedU8::new(b_raw);

    assert_eq!(a | b_raw, 0b11110011);
    assert_eq!(a | b_wrapped, 0b11110011);
}

#[test]
fn test_non_zero() {
    let a = NonZero::new(123u8).unwrap();
    let b = CheckedNum::new(a);

    assert_eq!(b | a, a)
}
