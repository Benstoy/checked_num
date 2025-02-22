# CheckedNum

Overflow-checked numbers for safety without sacrificing ergonomics.

## Usage

With `checked_num`

```rust
use checked_num::CheckedU16;

assert_eq!((CheckedU16::new(123) + 210) * 2, 666)
```

Without `checked_num`

```rust
assert!(
    123u16.checked_add(210)
        .and_then(|num| num.checked_mul(2))
        .and_then(|num| Some(num == 666))
        .is_some_and(|r| r)
);
```

## Features

- `#![no_std]` enabled per default.

- Supports `NonZero<_>` types for zero memory overhead.

- Only depends on `num-traits`.

- Supports checked versions of `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Shl`, `Shr` and `Neg` 

- Wraps `BitAnd`, `BitOr`, `BitXor` and `Inv` for convenience.

## Contributing

Areas for improvement:

- Implement the `?` operator.

- Implement checked casts.

- Introduce a macro that ensures all arithmetic operations are checked,
  preventing unintended unchecked operations due to precedence issues.

- Implement `num_traits::CheckedEuclid` and `num_traits::MulAdd`.

- Add `_Assign` variants for supported operations.

- Expand documentation.

- Add more tests.

## Limitations

Due to the orphan rule, `CheckedNum` types must appear on the left-hand side of mixed-type operations:

```rust
use checked_num::CheckedU16;

let a = CheckedU16::new(123);
let b = 210;

assert_eq!(a + b, 333) // correct
```

```rust
use checked_num::CheckedU16;

let a = CheckedU16::new(123);
let b = 210;

assert_eq!(b + a, 333) // fails to compile
```
