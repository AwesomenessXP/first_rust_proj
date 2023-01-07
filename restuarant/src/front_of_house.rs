// hosting and serving are children of front_of_house

// hosting is a sibling of serving
pub mod hosting;

// serving is a sibling of hosting
mod serving {
  fn take_order() {}
  fn serve_order() {}
  fn take_payment() {}
}