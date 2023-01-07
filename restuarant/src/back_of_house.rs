fn fix_incorrect_order() {
  cook_order();
  super::deliver_order();
}

fn cook_order() {}

pub struct Breakfast {
  pub toast: String,
  seasonal_fruit: String,
}

impl Breakfast {
  pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
          toast: String::from(toast), // this is public
          seasonal_fruit: String::from("peaches"), // this is private
      }
  }
}

pub enum Appetizer {
  Soup, 
  Salad,
}