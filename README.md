# CheckedNum

Overflow-checked numbers for safety that isn't an ergonomic nightmare.

---

## Usage

With `checked_num`

```rust
use checked_num::CheckedU16;

assert_eq!((CheckedU16::new(123) + 210) * 2, 666)
```

Without `checked_num`

```rust
assert!(
    a.checked_add(210)
        .and_then(|num| num.checked_mul(2))
        .and_then(|num| Some(num == 666))
        .is_some_and(|r| r)
);
```

## Features

- `#![no_std]` is enabled per default.

- Supports `NonZero<_>` types for 0 memory overhead.

- `num-traits` is the only dependency

- Supports checked versions of `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Shl`, `Shr` and `Neg` 

- Wraps `BitAnd`, `BitOr`, `BitXor` and `Inv` for convenience.

## Contributing

A few things that could still be improved:

- Implement the `?` operator.

- Implement checked casts.

- Add macro that checks whether every arithmetic operation is actually checked.
  (to avoid accidentally using unchecked operations due to confusing operator precendence.)

- Implement `num_traits::CheckedEuclid` and `num_traits::MulAdd`.

- Implement `_Assign` operations.

- Add more documentation.

- Add more tests.

## Limitations

Duo to the orphan rule you can only add normal numbers to the checked numbers not the other way around

```rust
use checked_num::CheckedU16; 
let a = CheckedU16::new(123);
let b = 210; 
assert_eq!(a + b, 333) // correct
```

```compile_fail
use checked_num::CheckedU16;

let a = CheckedU16::new(123);
let b = 210;

assert_eq!(b + a, 333) // fails to compile
```
