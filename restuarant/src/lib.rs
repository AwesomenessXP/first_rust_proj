// lib.rs is called a crate root

// if we replace the semicolon with brackets, this is the same thing as saying, import ALL code from front_of_house.rs
mod front_of_house;

fn deliver_order() {}

pub mod back_of_house;

pub use crate::front_of_house::hosting; // create a namespace for hosting
use crate::front_of_house::hosting::add_to_waitlist; // this is also valid

// mark this fn as public
pub fn eat_at_restaurant() {
    // Absolute path:
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path:
    front_of_house::hosting::add_to_waitlist();

    // order a breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we want
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // this line wont run bc seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // we can directly use hosting because of the "use" keyword
    hosting::add_to_waitlist();

    // we can also call this when this function is added to "use"
    add_to_waitlist();
}
