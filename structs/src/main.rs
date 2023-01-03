// struct
struct User {
    active: bool, // this is a field
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct BuildUser {
    email: String,
    username: String,
}

fn main() {
    // we dont have to specify fileds in the same order!!
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1, 
    };
    // // use dot notation to get a value:
    // user1.email = String::from("anotheremail@example.com");

    // -------- CREATING A NEW INSTANCE FROM ANOTHER INSTANCE -------------

    // short way: 
    let user2 = User {
        email: String::from("another@example.com"), 
        ..user1 // use .. to implicitly copy fields from user 1
    };

    let userEmailAndData = BuildUser {
        email: user2.email,
        username: user2.username,
    };

    println!("{}", userEmailAndData.username);
    // println!("{}", user2.email) // invalid bc ownership is transferred to userEmailAndData
    // println!("{}, user1.username"); this is invalid bc ownership is transferred to user2


    // ---------------------- TUPLE STRUCTS ---------------------------------
    // instances of structs:
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like structs
    let subject = AlwaysEqual;

    // -------------- OWNERSHIP OF STRUCTS -----------------------------------
    // lifetimes: ensure that the data referenced by a struct is valid for as long as the struct is in scope
    // structs can store references owned by something else, but requre lifetimes (needs a lifetime specifier)
    // lifetimes are needed when storing &str!

    // -------------- EXAMPLE PROGRAM WITH STRUCTS ----------------------------
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1),
    );

    // println macro can only properly print primitives
    // structs are not primitive, and you can print them so many ways
    // use {:?} to use an output format Debug
    println!("rect1 is {:#?}", rect1);
}

// instead of passing two parameters, you can use tuples!
fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
