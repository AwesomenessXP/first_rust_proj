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
        rect1.area(),
    );

    // -------------- DEBUGGING STRUCTS ------------------------------------
    // println macro can only properly print primitives
    // structs are not primitive, and you can print them so many ways
    // use {:?} to use an output format Debug
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {:#?}", rect1); // worse way of debugging
    dbg!(&rect1); // better way of debugging
    dbg!(30 * scale);

    // --------- METHODS IN STRUCTS ---------------------------------------
    // you can define methods inside structs
    println!(
        "The area of the rectangle is {} squre pixels.",
        rect1.area(),
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width: {}", rect1.width);
    }

    // ---------- METHODS WITH MORE PARAMS ---------------------------------
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10, 
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3?: {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);
    dbg!(sq);
}

// instead of passing two parameters, you can use tuples!
fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// this is an implementation block
impl Rectangle {
    // you NEED &self as the first parameter

    // ---- METHODS -------------
    fn area(&self) -> u32 { 
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // -------- ASSOCIATED CLASSES --------
    // Self is a keyword that is an alias for Rectangle 
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
