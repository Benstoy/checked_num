use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};

use num_traits::Inv;
use num_traits::ops::checked::*;

use crate::{CheckedU32, builtin_int::BuiltinInt};

/// Overflow-Checked Number.
/// Can be used like any other integer type.
///
/// # Operations with non-checked types
/// Integer types of the same bitsize can be used in binary operations
/// with `CheckedNum`, as long as they appear on the right-hand side.
///
/// These calculations will behave exactly the same
/// as when performed between two `CheckedNum` values.
///
/// Example:
/// ```rust
/// use checked_num::CheckedU16;
///
/// let a = CheckedU16::new(123);
/// let b = 210;
///
/// assert_eq!(a + b, 333) // correct
/// ```
///
/// ```compile_fail
/// use checked_num::CheckedU16;
///
/// let a = CheckedU16::new(123);
/// let b = 210;
///
/// assert_eq!(b + a, 333) // fails to compile
/// ```
///
/// This is a rust limitation that cannot be overcome.
///
/// # Overflow
/// In case of an overflow the value is discarded.
/// The error will be propagated in all subsequent calculations (similar to NaN in floats).
///
/// Example:
/// ```rust
/// use checked_num::CheckedI8;
///
/// let a = CheckedI8::new(100);
/// let b = CheckedI8::new(100);
/// let c = CheckedI8::new(100);
///
/// assert!((a + b - c).did_overflow());
///
/// ```
///
/// With overflowing behavior this would instead correctly result in 100
///
/// # Equality
/// Overflowed values are never equal.
///
/// Example:
/// ```rust
/// use checked_num::CheckedU8;
///
/// let a = CheckedU8::new(u8::MAX);
/// let b = CheckedU8::new(1);
///
/// assert_ne!(a + b, a + b);
/// ```
#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct CheckedNum<T: CheckedNumTraits>(Option<T>);

// This bound is purposfully restrictive to avoid breaking changes
pub trait CheckedNumTraits: BuiltinInt {}
impl<T: BuiltinInt> CheckedNumTraits for T {}

impl<T: CheckedNumTraits> CheckedNum<T> {
    const OVERFLOWED: Self = Self(None);

    pub fn new(num: T) -> Self {
        Self(Some(num))
    }

    pub fn as_option(self) -> Option<T> {
        self.0
    }

    pub fn did_overflow(&self) -> bool {
        self.as_option().is_none()
    }
}

impl<T: CheckedNumTraits> From<T> for CheckedNum<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T: CheckedNumTraits> From<Option<T>> for CheckedNum<T> {
    fn from(maybe_num: Option<T>) -> Self {
        CheckedNum(maybe_num)
    }
}

impl<T: CheckedNumTraits> Iterator for CheckedNum<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.as_option().take()
    }
}

impl<T: CheckedNumTraits + PartialEq<B>, B: BuiltinInt> PartialEq<B> for CheckedNum<T> {
    fn eq(&self, rhs: &B) -> bool {
        self.as_option().is_some_and(|num| num.eq(rhs))
    }
}

impl<T: CheckedNumTraits + PartialEq<B>, B: CheckedNumTraits + BuiltinInt> PartialEq<CheckedNum<B>>
    for CheckedNum<T>
{
    fn eq(&self, rhs: &CheckedNum<B>) -> bool {
        rhs.as_option().is_some_and(|num| self.eq(&num))
    }
}

impl<T: CheckedNumTraits + PartialOrd<B>, B: BuiltinInt> PartialOrd<B> for CheckedNum<T> {
    fn partial_cmp(&self, other: &B) -> Option<Ordering> {
        self.as_option().and_then(|num| num.partial_cmp(other))
    }
}

impl<T: CheckedNumTraits + PartialOrd<B>, B: CheckedNumTraits + BuiltinInt>
    PartialOrd<CheckedNum<B>> for CheckedNum<T>
{
    fn partial_cmp(&self, rhs: &CheckedNum<B>) -> Option<Ordering> {
        rhs.as_option()
            .and_then(|rhs_num| self.partial_cmp(&rhs_num))
    }
}

macro_rules! impl_op {
    ($trait:ident, $trait_fn:ident) => {
        impl<T: CheckedNumTraits + $trait<B, Output = T>, B: BuiltinInt> $trait<B>
            for CheckedNum<T>
        {
            type Output = Self;

            fn $trait_fn(self, rhs: B) -> <Self as $trait<B>>::Output {
                self.as_option().map_or(CheckedNum::OVERFLOWED, |num| {
                    CheckedNum::new(num.$trait_fn(rhs))
                })
            }
        }

        impl<T: CheckedNumTraits + $trait<B, Output = T>, B: CheckedNumTraits + BuiltinInt>
            $trait<CheckedNum<B>> for CheckedNum<T>
        {
            type Output = Self;

            fn $trait_fn(self, rhs: CheckedNum<B>) -> <Self as $trait<CheckedNum<B>>>::Output {
                rhs.as_option()
                    .map_or(CheckedNum::OVERFLOWED, |num| self.$trait_fn(num))
            }
        }
    };

    ($trait:ident, $checked_trait:ident, $trait_fn:ident, $checked_fn:ident) => {
        impl<T: CheckedNumTraits + $checked_trait> $trait<T> for CheckedNum<T> {
            type Output = Self;
            fn $trait_fn(self, rhs: T) -> <Self as $trait>::Output {
                self.as_option()
                    .map_or(Self::OVERFLOWED, |num| num.$checked_fn(&rhs).into())
            }
        }

        impl<T: CheckedNumTraits + $checked_trait> $trait for CheckedNum<T> {
            type Output = Self;
            fn $trait_fn(self, rhs: Self) -> <Self as $trait>::Output {
                rhs.as_option()
                    .map_or(Self::OVERFLOWED, |num| self.$trait_fn(num))
            }
        }
    };
}

macro_rules! impl_shift_op {
    ($trait:ident, $checked_trait:ident, $trait_fn:ident, $checked_fn:ident) => {
        impl<T: CheckedNumTraits + $checked_trait, B: Into<CheckedU32>> $trait<B>
            for CheckedNum<T>
        {
            type Output = Self;

            fn $trait_fn(self, rhs: B) -> <Self as $trait<B>>::Output {
                self.as_option().map_or(Self::OVERFLOWED, |num| {
                    rhs.into()
                        .as_option()
                        .map_or(Self::OVERFLOWED, |rhs_num| Self(num.$checked_fn(rhs_num)))
                })
            }
        }
    };
}

// Missing from num_traits:
// - To/From bytes for CheckedNum<u8>
// - Euclid calculations
// - MulAdd

impl_op! {Add, CheckedAdd, add, checked_add}
impl_op! {Sub, CheckedSub, sub, checked_sub}
impl_op! {Mul, CheckedMul, mul, checked_mul}
impl_op! {Div, CheckedDiv, div, checked_div}
impl_op! {Rem, CheckedRem, rem, checked_rem}
impl_shift_op! {Shl, CheckedShl, shl, checked_shl}
impl_shift_op! {Shr, CheckedShr, shr, checked_shr}

impl_op! {BitAnd, bitand}
impl_op! {BitOr, bitor}
impl_op! {BitXor, bitxor}

impl<T: CheckedNumTraits + CheckedNeg> Neg for CheckedNum<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.as_option()
            .map_or(Self::OVERFLOWED, |num| CheckedNum(num.checked_neg()))
    }
}

impl<T: CheckedNumTraits + Inv<Output = T>> Inv for CheckedNum<T> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        self.as_option()
            .map_or(Self::OVERFLOWED, |num| CheckedNum::new(num.inv()))
    }
}
