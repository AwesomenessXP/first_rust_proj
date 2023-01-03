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

# Ownership
Ownership has a set of rules that the compiler checks when managing memory
Rules of Ownership: 
- each value has an owner
- there can only be ONE owner
- when owner goes out of scope -> value is dropped

Instead of transferring ownership and returning a value to scope, we can use *references* to the value that owns something in memory

# References
When making references, you can either:
- make ONE mutable reference
- or make MANY immutable references

Why is this? -> It prevents a *data race*: 
- two or more ptrs accessing same data at the same time
- one of the ptrs used to write the data
- NO mechanism to syncrhonize access to data!
- cause undefined behavior

Immutable references don't expect the value to change!

# Slices
A string slice refrences part of a `String`

Slice ranges MUST occur at valid UTF-8 character boundaries
You cannot create a slice in the middle of a multibyte character

Literal string slices are immutable (&str by default)

Slice syntax:
```rust
// solution: string slice
let s = String::from("hello world");
// make 2 immutable references to s
let hello = &s[0..5];
let world = &s[6..11];

// range syntax
// these are the same: 
let slice = &s[0..2];

// if the slice contains last byte:
let len = s.len();
let slice = &s[3..len];
let slice = &s[3..]; // this is same as the last line!

// if the slice has the ENTIRE string:
let len = s.len();
let slice = &s[0..len];
let slice = &s[..]; // same as last line
```
