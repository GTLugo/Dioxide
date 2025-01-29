use crate::{error::OsError, std::*};

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

  pub fn run(self) -> Result<(), OsError> {
    // println!("bau bau");
    Ok(())
  }
}
