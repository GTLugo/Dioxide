use dioxide_std::println;

use super::framebuffer::*;
use crate::error::OsError;

pub struct OS {
  pub framebuffer: FrameBuffer<'static>,
}

impl OS {
  pub fn run(self) -> Result<(), OsError> {
    println!("bau bau");

    fn f() {
      f();
    }
    f();

    Ok(())
  }
}
