// get the Asparagus struct from inside a submodule
use crate::garden::vegetables::Asaparagus;

// include code from src/garden.rs
// this is how you include garden.rs inside the crate root!
pub mod garden; 

fn main() {
    let plant = Asaparagus {};
    println!("I'm growing {:?}!", plant);
}
