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
        SpreadsheetCell::Int(3);
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
