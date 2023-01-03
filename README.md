# Integer Types
- a *scalar* type is a single value: ints, floats, bools, chars
- 8 bit: i8, u8
- 16-bit: i16, u16
...
- 128-bit: i128, u128
- arch: isize, usize

# Handling overflow
Rust does two's complement wrapping (value wraps around)
wrap uses `wrapping_*`