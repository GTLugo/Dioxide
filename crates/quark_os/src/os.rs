use crate::std::*;

pub struct OS {}

impl Default for OS {
  fn default() -> Self {
    Self::new()
  }
}

impl OS {
  pub fn new() -> Self {
    Self {}
  }

  pub fn run(self) {
    println!("bau bau");
  }
}
