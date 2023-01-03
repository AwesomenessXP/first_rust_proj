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

Literal string slices are immutable (`&str` by default)

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

# Structs
Structs are objects that hold multiple values. Creating an instance of a struct doesn't need fields in the same order.

All fields in a struct instance *must* be mutable!

##### Creating a new instance from another instance
```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1, 
};

// long way: 
let user2 = User {
  active: user1.active,
  username: user1.username,
  email: String::from("another@example.com"),
  sign_in_count: user1.sign_in_count,
};

// short way: 
let user2 = User {
  email: String::from("another@example.com"),
  ..user1 // use .. to implicitly copy fields from user 1
};
```
From the example, `user1.username` is now invalid because it transferred ownership to `user2.username`. However, `user1.email` is valid, because it DID NOT transfer ownership to `user2`. If we specified a different String for the username in `user2`, then `user1.email` would still be valid!

Because the fields `active` and `sign_in_count` are fixed, and stored on the stack, the implement the `Copy` trait, so they are copied, rather than moved

##### Tuple Structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```
Just like with tuples, you can destructure tuple structs, and use `.` to access its fields

Unit-like structs look like this:
```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```
You use this when you don't have data to store in the struct.

##### Displaying structs
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("rect1 is {}", rect1);
```
`println` macro can only properly print primitives. Structs are not primitive, and you can print them so many ways.
Use `{:#?}` to use an output format `Debug`, but to do that, you need to specify an attribute `[derive(Debug)]`

*You can also use `{:?}`, but the other format looks nicer* 

Ex: 
```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let rect1 = Rectangle {
      width: 30,
      height: 50,
  };

  println!("rect1 is {:?}", rect1);
}
```