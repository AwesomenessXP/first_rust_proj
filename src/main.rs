fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
// create project with 'cargo new'
// build project with 'cargo build'
// build and run proj with 'cargo run'
// build proj without producing .exe: 'cargo check'
