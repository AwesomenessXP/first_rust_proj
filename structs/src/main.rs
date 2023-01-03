struct User {
    active: bool, // this is a field
    username: String,
    email: String,
    sign_in_count: u64,
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

    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    // field init shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
