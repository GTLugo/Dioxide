use super::framebuffer::FrameBuffer;
use crate::error::OsError;

pub struct OS {
  pub framebuffer: FrameBuffer<'static>,
}

impl OS {
  pub fn run(mut self) -> Result<(), OsError> {
    self.framebuffer.clear_screen();
    // println!("{:#?}", self.framebuffer.info());

    // for byte in framebuffer.iter_mut() {
    //   *byte = 0x69;
    // }
    // for (x, y) in (20..100).flat_map(|x| (30..100).map(move |y| (x, y))) {
    //   let position = framebuffer::Position { x, y };
    //   let color = framebuffer::Color { r: 0, g: 255, b: 0 };

    //   framebuffer.set_pixel(position, color)?;
    // }

    // const VGA_BUFFER: *mut u8 = 0xb8000 as _;
    // static HELLO: &[u8] = b"Hello World!";
    // for (i, &byte) in HELLO.iter().enumerate() {
    //   unsafe {
    //     *VGA_BUFFER.offset(i as isize * 2) = byte;
    //     *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
    //   }
    // }

    // println!("bau bau");
    Ok(())
  }
}
