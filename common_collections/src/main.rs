fn main() {
    // to create a new vector
    let v: Vec<i32> = Vec::new();

    // you can also use a macro that initalizes the vector
    let v = vec![1, 2, 3];

    // updating a vector:
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // -------- READING ELEMENTS OF VECTORS -----------------------
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // accessing past the boundaries gives us errors
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // use this if you want the program to crash if error
    let does_not_exist = v.get(100);// use this to return none without panicking

    // -------------------- MUTABILITY ----------------------------------------
    // Mutable and immutable references conflict and cause error

    // ------------ ITERATING OVER VALUES ------------------------------------
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // storing enum variants in a vector:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    // --------------- STRINGS -----------------------------------------------
    // &str: string slice
    // String: growable, mutable, owned, string type

    // making a new string:
    let mut s = String::new();

    // appending to a string

    // using "from"
    let mut s = String::from("foo");
    s.push_str("bar");

    // using string literal
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // we might accidentally move the scope of s2 here!
    println!("s2 is {s2}");

    // ----------------- CONCATENATION --------------------------------------

    // using string literals
    let s = "hey".to_string();
    let s2 = " there".to_string();
    let s4 = " world".to_string();
    let s3 = s + &s2 + &s4 + "!"; // the result is a String type
    println!("s3: {s3}");

    // using "format!" macro
    let s3 = format!("{s3}-{s2}-{s3}");
    println!("{s3}");

    let s5 = "sdfsdf";
    let s = &s5[0..1];
    println!("{s}");

    // ------------ ITERATING OVER STRINGS ----------------------------------
    let s = "sdflkjwerlkjsdf";
    for c in s.chars() {
        println!("{c}");
    }
}
