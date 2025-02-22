use core::num::NonZero;

/// All built-in Integer types
///
/// Excludes `Wrapping<_>``
pub trait BuiltinInt: Copy {}

impl BuiltinInt for i128 {}
impl BuiltinInt for i64 {}
impl BuiltinInt for i32 {}
impl BuiltinInt for i16 {}
impl BuiltinInt for i8 {}
impl BuiltinInt for u128 {}
impl BuiltinInt for u64 {}
impl BuiltinInt for u32 {}
impl BuiltinInt for u16 {}
impl BuiltinInt for u8 {}

impl BuiltinInt for NonZero<i128> {}
impl BuiltinInt for NonZero<i64> {}
impl BuiltinInt for NonZero<i32> {}
impl BuiltinInt for NonZero<i16> {}
impl BuiltinInt for NonZero<i8> {}
impl BuiltinInt for NonZero<u128> {}
impl BuiltinInt for NonZero<u64> {}
impl BuiltinInt for NonZero<u32> {}
impl BuiltinInt for NonZero<u16> {}
impl BuiltinInt for NonZero<u8> {}

// Wrapping<T> is purposfully ignored!
// Adding checked to wrapping values does not make sense.
//
// impl<T: BuiltinInt> BuiltinInt for Wrapping<T> {}
