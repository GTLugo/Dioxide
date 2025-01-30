use bootloader_api::{
  BootInfo,
  info::{FrameBufferInfo, PixelFormat},
};

use crate::error::OsError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

pub struct FrameBuffer {
  buffer: &'static mut [u8],
  info: FrameBufferInfo,
}

impl FrameBuffer {
  pub fn new(boot_info: &mut BootInfo) -> Option<Self> {
    boot_info.framebuffer.take().map(|framebuffer| {
      let info = framebuffer.info();
      let buffer = framebuffer.into_buffer();
      Self { buffer, info }
    })
  }

  pub fn set_pixel(&mut self, position: Position, color: Color) -> Result<Color, OsError> {
    // calculate offset to first byte of pixel
    let byte_offset = {
      // use stride to calculate pixel offset of target line
      let line_offset = position.y * self.info.stride;
      // add x position to get the absolute pixel offset in buffer
      let pixel_offset = line_offset + position.x;
      // convert to byte offset
      pixel_offset * self.info.bytes_per_pixel
    };

    if byte_offset >= self.buffer.len() {
      return Err(OsError::OutOfBounds(byte_offset));
    }

    // set pixel based on color format

    let pixel_bytes = self
      .buffer
      .get_mut(byte_offset..(byte_offset + self.info.bytes_per_pixel))
      .ok_or(OsError::OutOfBounds(byte_offset))?;
    #[allow(clippy::get_first)]
    match self.info.pixel_format {
      PixelFormat::Rgb => {
        // SAFETY: (taken from rustc) The underlying array of a slice can be reinterpreted as an actual array `[T; N]` if `N` is not greater than the slice's length.
        let [r, g, b] = unsafe { &mut *(pixel_bytes.as_mut_ptr() as *mut [u8; 3]) };

        *r = color.r;
        *g = color.g;
        *b = color.b;

        Ok(Color { r: *r, g: *g, b: *b })
      }
      PixelFormat::Bgr => {
        let [b, g, r] = unsafe { &mut *(pixel_bytes.as_mut_ptr() as *mut [u8; 3]) };

        *r = color.r;
        *g = color.g;
        *b = color.b;

        Ok(Color { r: *r, g: *g, b: *b })
      }
      PixelFormat::U8 => {
        let [value] = unsafe { &mut *(pixel_bytes.as_mut_ptr() as *mut [u8; 1]) };

        // use a simple average-based grayscale transform
        *value = color.r / 3 + color.g / 3 + color.b / 3;

        Ok(Color {
          r: *value,
          g: *value,
          b: *value,
        })
      }
      other => panic!("unknown pixel format `{other:?}`"),
    }
  }

  pub fn iter(&self) -> FrameBufferIterator {
    FrameBufferIterator {
      framebuffer: self,
      index: 0,
    }
  }

  pub fn iter_mut(&mut self) -> FrameBufferIteratorMut {
    FrameBufferIteratorMut {
      framebuffer: self,
      index: 0,
    }
  }
}

pub struct FrameBufferIterator<'a> {
  framebuffer: &'a FrameBuffer,
  index: usize,
}

impl<'a> Iterator for FrameBufferIterator<'a> {
  type Item = &'a u8;

  fn next(&mut self) -> Option<Self::Item> {
    self.framebuffer.buffer.get(self.index).inspect(|_| {
      self.index += 1;
    })
  }
}

pub struct FrameBufferIteratorMut<'a> {
  framebuffer: &'a mut FrameBuffer,
  index: usize,
}

impl<'a> Iterator for FrameBufferIteratorMut<'a> {
  type Item = &'a mut u8;

  fn next(&mut self) -> Option<Self::Item> {
    match self.framebuffer.buffer.get_mut(self.index) {
      Some(byte) => {
        self.index += 1;
        // Need to reinterpret the lifetime... yabe.
        unsafe { core::ptr::from_mut(byte).as_mut() }
      }
      None => None,
    }
  }
}

impl<'a> IntoIterator for &'a FrameBuffer {
  type IntoIter = FrameBufferIterator<'a>;
  type Item = &'a u8;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut FrameBuffer {
  type IntoIter = FrameBufferIteratorMut<'a>;
  type Item = &'a mut u8;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}
