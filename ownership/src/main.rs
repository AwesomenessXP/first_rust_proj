fn main() {
    // string literals are immutable!
    // String type allocates data on the heap, str is stored in the stack
    let mut s = String::from("Hello");
    s.push_str(", world!"); // append a literal to a string
    println!("{s}");

    // ---------- BASICS OF OWNERSHIP ----------------------------------

    // integers are fixed, known sizes -> stored in stack
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // s1 "owns" the memory storing "hello" in the heap
    let s1 = String::from("hello");
    // deeply copy s1 to s2 so that s1 doesnt accidentally transfer ownership to s2 (meaning s1 is no longer in scope)
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    // --------- TRANSFERRING OWNERSHIP TO A DIFFERENT SCOPE-------------
    let s = String::from("hello"); // s is in scope
    takes_ownership(s); // s's value moved into function, no longer valid

    let x = 5; // x is in scope
    makes_copy(x); // x moves inside function, but it is type i32 and is COPIED, so x is still within scope

    // returning ownership to this scope:
    let s1 = gives_ownership(); // fn moves return value to s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2);
    // when the scope ends:
    // s1 drops its value
    // s2 transfers ownership to s3, so does nothing
    // s3 transfers ownership, gets ownership again, then drops its value


    // ----------- REFERENCES AND BORROWING ----------------------------
    let s1 = String::from("hello"); // s1 comes to scope
    // make a *reference* to s1, but not transfer ownership to a diff scope!
    let len = calculate_length(&s1); 
    println!("The length of '{}' is {}", s1, len);

    // when making references:
    // can make ONE mutable reference
    // or MANY immutable references
    let mut s = String::from("hello");
    change(&mut s);

    // we can create a new scope and use a mutable reference in that scope
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; // this is valid bc its in another scope
    println!("r2: {r2}");


    // --------- DANGLING POINTERS ------------------------------------
    let reference_to_nothing = dangle();

    // --------------- SLICE TYPES ------------------------------------

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

    // example:
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    // s.clear(); // error! clear is a *mutable* reference
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // ----------- STRING LITERALS AS SLICES ------------------------
    let s = "Hello, world!"; // here, s is &str -> so its immutable
    // str can be String, but String CANNOT be str!
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes
    // i = index, &item = byte in tuple
    // iter() -> returns each element in a collection
    // enumerate() -> returns an index, and value in a tuple -> (i, value)
    for (i, &item) in bytes.iter().enumerate() {
        // look for the first space in the string
        if item == b' ' { // b' ' is an ASCII byte literal with a value of ' ' 
            return &s[0..i];
        }// if
    }// for
    &s[..]
}

fn dangle() -> String {
    let s = String::from("hello");
    // &s -> once s is out of scope, it references nothing!
    s
}

fn change(some_string: &mut String) {
    // REMEMBER: make the string mutable first!
    some_string.push_str(", world");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}