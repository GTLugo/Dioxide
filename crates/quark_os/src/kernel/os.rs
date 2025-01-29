use bootloader_api::{info::FrameBuffer};

use crate::{error::OsError, std::*};

pub struct OS {
  pub framebuffer: Option<FrameBuffer>,
}

impl OS {
  pub fn run(mut self) -> Result<(), OsError> {
    if let Some(framebuffer) = self.framebuffer.as_mut() {
      for byte in framebuffer.buffer_mut() {
        *byte = 0x90;
      }
    }
    
    // println!("bau bau");
    Ok(())
  }
}
