fn main() {
    // create project with 'cargo new'
    // build project with 'cargo build'
    // build and run proj with 'cargo run'
    // build proj without producing .exe: 'cargo check'

    // ------- INTEGER TYPES --------------------
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // -------- FLOATING POINT TYPES ---------------
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // --------- NUMERIC OPERATIONS ---------------
    // addition
    let sum = 5 + 10;
    // subtraction 
    let difference = 95.5 - 4.3;
    // mult
    let product = 4 * 30;
    // div
    let quotient = 56.7 / 32.2;
    let truncated = -5/3; // results in -1
    // remainder
    let remainder = 43 % 5;

    // ----------- BOOLEANS --------------------
    let t = true;
    let f: bool = false; // with explicit type annotation

    // ----------- CHARACTER TYPE ----------------
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("The value of cat is: {heart_eyed_cat}");

    // ----------- TUPLE TYPE ---------------------
    let tup: (i32, f64, u8) = (500, 6.4, 1); // cannot change size
    let (first, second, third) = tup; // destructuring the tuple
    println!("The value of y: {}", second);
    // to access tuple elements, use period, then their indexes:
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // ------------ ARRAY TYPE ----------------------  
    let a = [1, 2, 3, 4, 5]; // stored on the stack, not heap

    // i32 is the type of element
    // 5 is the array length
    let b: [i32; 5] = [1, 2, 3, 4, 5]; 
    let c = [3; 5]; // same as: [3, 3, 3, 3, 3];
    let first = a[0];
    let second = b[1];

    // ------------ FUNCTIONS -------------------------
    another_function(5, 'h');

    // ------------- EXPRESSIONS ----------------------
    // REMEMBER: expressions dont end in semicolons!
    let hey = {
        let lol = 3;
        lol + 1 // DONT ADD SEMICOLON HERE!!
    };

    println!("hey: {hey}");

    // ------------- FUNCS WITH RETURN VALUES -----------
    let x = five();  
    println!("five() = {x} ");

    let x = plus_one(x);

    println!("The value of x is: {x}")
}

// YOU MUST include type of the parameters!
fn another_function(x: i32, unit: char) {
    println!("The value of x is: {x}, value of unit is: {unit}");
}

fn five() -> i32 {
    // early returns use return keyword
    5 // last expression is implicitly returns, DONT USE semicolon bc its an expression not an error
}

fn plus_one(x: i32) -> i32 {
    x + 1 // this is an expression, not a statement, DONT USE SEMICOLON
}
