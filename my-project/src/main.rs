use crate::garden::vegetables::Asaparagus;

pub mod garden;

fn main() {
    let plant = Asaparagus {};
    println!("I'm growing {:?}!", plant);
}
