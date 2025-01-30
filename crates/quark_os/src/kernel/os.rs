use super::framebuffer::{self, FrameBuffer};
use crate::{error::OsError, std::*};

pub struct OS {
  pub framebuffer: Option<FrameBuffer>,
}

impl OS {
  pub fn run(mut self) -> Result<(), OsError> {
    if let Some(framebuffer) = self.framebuffer.as_mut() {
      for byte in framebuffer.iter_mut() {
        *byte = 0x69;
      }

      for (x, y) in (20..100).flat_map(|x| (30..100).map(move |y| (x, y))) {
        let position = framebuffer::Position { x, y };
        let color = framebuffer::Color { r: 0, g: 255, b: 0 };

        framebuffer.set_pixel(position, color)?;
      }
    }
    
    // println!("bau bau");
    Ok(())
  }
}
