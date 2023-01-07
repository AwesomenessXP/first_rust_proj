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

## Creating a new instance from another instance
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

## Tuple Structs
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Just like with tuples, you can destructure tuple structs, and use `.` to access its fields

Unit-like structs look like this:
```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```
You use this when you don't have data to store in the struct.

## Displaying structs
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

  println!("rect1 is {:#?}", rect1);
}
```

A better way of debugging is using `dbg!` macro
```rust
dbg!(&rect1);
```

## Method syntax
A method is similar to a function. They can only be defined in a struct, enum, or trait object
```rust
// this is an implementation block
impl Rectangle {
  // this is a method
  fn area(&self) -> u32 {
      self.width * self.height
  }
}
```
IMPORTANT: METHODS MUST ALWAYS HAVE `&self` as the first parameter. `&self` is an alias for whatever `impl` is for. In this case, it means the `Rectangle` struct.

## Associated Functions
An *associated function* is also inside a `impl` block but doesn't have `&self` as the first parameter.
```rust
// Self is a keyword that is an alias for Rectangle 
fn square(size: u32) -> Self {
  Self {
      width: size,
      height: size,
  }
}

let sq = Rectangle::square(3);
```
Use `::` to call associated functions.

# Enums
Enums: allow you to define a type by enumerating its possible *variants*
```rust
enum IpAddr { // this is the identifier
  V4(String), // this is a variant, tells us what kind of ip addr it is
  V6(String),
}

enum IpAddr { 
    V4(u8, u8, u8, u8),
    V6(String),
}
```
Structs are like 'AND' combination of all fields. Enums are like 'OR' of individual variants. You can include enums in other enums.
```rust
// enums can have different types:
enum Message {
    Quit, // no data
    Move {x: i32, y: i32}, // struct
    Write(String), // String
    ChangeColor(i32, i32, i32), // tuple structs
}
```
The enum above is the same as saying the bottom:
```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```
Notice how they are all variations (or *variants*) of the same type (Message), so why not use an enum to represent all the variants?

Enums, like structs, can also define methods using `impl`:
```rust
// -- snip ---
impl Message {
  fn call(&self) {
    // method body here
  }
}
let m = Message::Write(String::from("hello"));
m.call();
```

Rust does not have nulls, but has `Option<T>` which tells you if a value is present or not.
```rust
enum Option<T> {
  None, // null
  Some(T), // not null
}
```

This gives us an error because `i8` is a different type than `Option<i8>`. With `Option<i8>`, there might not even be a *value* for it! So you have to convert `Option<T>` to `T` before actually using it!
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
```

## Matching with `Option<T>`
A common pattern in Rust is using `match` against an enum, binding a variable to the data inside an enum, then executing code.
How do we convert `Option<T>` to `T`?

*Matches must be exhaustive.* If you have an arm Some(i), you also need a None arm. This means that matches are *exhuastive*: cover every valid possibility.

## Catch-All Patterns
Catch-all is like `default` value in a switch statement. 

To catch-all and not bind to a value in an arm: `_`

If you don't want the catch-all to do anything, return `()` (or nothing):
```rust
let dice_roll = 9;
match dice_roll {
  _ => (),
}
```

## if let
You use this when you want to only match one pattern, and ignore the rest

```rust
// long way
match config_max {
  Some(max) => println!("The maximum is configured to be {}", max),
  _ => (),
}

// if let way
if let Some(max) = config_max {
  println!("The maximum is configured to be {}", max);
}
else { 
  // dont do anything
}
```

# Modules
This is how we organize code.

Lets have a crate, backyard, contain all these files and directories:

```rust
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```


## Start from crate root
Look for crate root (src/lib.rs or src/main.rs)

## Declare modules
To declare a new module: `mod garden;`

## Declare submodules
To declare submodules: `mod vegetables;` (same as modules)

## Path to code in modules
Once you have a module in your crate, you can refer to code from anywhere in the same crate. Ex: an `Asparagus` would be found in `crate::garden::vegetables::Asparagus`

## Creating a library crate
Run `cargo new library_name --lib`

## Nesting modules
You can nest modules inside other modules with brackets

Modules can hold definitions for structs, enums, constants, traits, and functions (like `.hpp` files)

## Absolute and Relative paths
```rust
// mark this fn as public
pub fn eat_at_restaurant() {
  // Absolute path:
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path:
  front_of_house::hosting::add_to_waitlist();
}
```
Because `front_of_house` is already at the same level as `eat_at_restaurant` (its inside the file), you can use a relative path.

## Private and public modules
*Modules are also private by default.* Make sure you add `pub` to expose a module's inner code. You also need to add `pub` to the specific submodule to expose it. 

However, if you make an enum public, all of its variants are public.

Specifing `super` in the beginning of a relative path means you want to get a parent module.

The `use` keyword only makes a shortcut at that specific scope. If you `use` a module outside of its scope, it is *invalid*. This is because `use` is *private by default*. If you want to export it so that parent modules can use it, you need to specify `pub use`.

## Aliases
When bringing in a module with `use`, we can also use `as` to create an alias for the module:
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() => IoResult<()> {}
```

## Using nested paths
We can clean up the `use` statement if using nested items from the same module.

This is inefficient:
```rust
use std::cmp::Ordering;
use std::io;
```

Instead, we can use one line:
```rust
use std::{cmp::Ordering, io};
```

We can use nested paths at any level:
```rust
use std::io;
use std::io::Write;
```

We can use `self` so a module can include itself:
```rust
// These are the same thing
use std::io;
use std::io::{self, Write};
```

You only need to load a file with `mod` only once. After that, every other file has to refer to the loaded file's location.

# Collections
## Vectors
To make a new vector: 
```rust
let v: Vec<i32> = Vec::new();
```

To read elements, use `v.get(index)`. Another way is: `let third: &i32 = &v[index];`

Using the `get()` method is preferred because the program doesnt crash, and returns a `None` enum.

##### Mutability
You cannot have mutable and immutable references, as stated by ownership rules.
This will give an error:
```rust
let mut v = vec![1, 2, 3, 4, 5]; // this is mutable (remember we can only have one)

let first = &v[0]; // this is an immutable reference (conflicts with our mutable reference)

v.push(6);

println!("The first element is: {first}");
```
This shows you cannot have immutable AND mutable references to a vector at the same time. Vectors put elements next to each other in the same memory location. If we make a reference to the first element, and push a new element to the end, it would give an error because the vector allocates elements to a *new* memory location, and now the reference points to deallocated memory (the old element isnt there anymore).

##### Iterating over values
To iterate over immutable references. Same thing applies to mutable references with `&mut`.
```rust
for i in &v {
    println!("{i}");
}
```

##### Storing enum variants in vectors
We can store enum variants in vectors, but we must be explicit about their types. Remember, matches are exhaustive, so name every variant as an element. 

## Strings
There are two types of strings: string literals (`&str`), and the library `String`.

To make a string literal into a `String`: 
1. `let s = data.to_string();`
2. `let s = String::from("initial comments");`

##### Updating a string:
```rust
let mut s = String::from("foo");
s.push_str("bar"); // append to the end of a String
```

##### Concatenating two Strings:
You must transfer ownership of the first string to the new string, and append the next strings as references, or string literals.

Using `+` operator:
```rust
let s1 = String::from("Hello, ");
let s2 = String::from(" world!");
let s3 = s1 + &s2;
```

(PREFERRED) Using `format!` macro:
```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

##### Slicing strings:
You cannot index a `String` type in rust. However, you make a string slice.

We can use `[]` as a range tos make a string slice.
```rust
let s5 = "sdfsdf";
let s = &s5[0..4]; // contain the first 4 bytes, 0 - 3, exclude 4
println!("{s}");
```

Printing chars and bytes:
```rust
// chars
for c in "Зд".chars() {
    println!("{c}");
}

// bytes
for b in "Зд".bytes() {
    println!("{b}");
}
```

