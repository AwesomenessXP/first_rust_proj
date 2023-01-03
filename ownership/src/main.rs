fn main() {
    // string literals are immutable!
    // String type allocates data on the heap, str is stored in the stack
    let mut s = String::from("Hello");
    s.push_str(", world!"); // append a literal to a string
    println!("{s}");

    // ---------- BASICS OF OWNERSHIP ------------

    // integers are fixed, known sizes -> stored in stack
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // s1 "owns" the memory storing "hello" in the heap
    let s1 = String::from("hello");
    // deeply copy s1 to s2 so that s1 doesnt accidentally transfer ownership to s2 (meaning s1 is no longer in scope)
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // --------- TRANSFERRING OWNERSHIP TO A DIFFERENT SCOPE-------
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

    // ----------- REFERENCES AND BORROWING ---------------------
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

    // --------- DANGLING POINTERS ------------------
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
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