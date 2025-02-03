use core::ops::{Deref, DerefMut};

use bootloader_api::info::FrameBufferInfo;

use crate::platform::*;

pub trait PlatformLogger: Send + Sync {
  type Internal;

  fn new(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) -> Self
  where
    Self: Sized;

  fn internal(&self) -> &Self::Internal;
}

pub struct Logger {
  #[cfg(feature = "x86")]
  internal: x86::logger::X86Logger,
}

impl Logger {
  pub fn new(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) -> Self {
    #[cfg(feature = "x86")]
    let internal = x86::logger::X86Logger::new(framebuffer, framebuffer_info);
    Self { internal }
  }
}

impl Deref for Logger {
  #[cfg(feature = "x86")]
  type Target = x86::logger::X86Logger;

  fn deref(&self) -> &Self::Target {
    &self.internal
  }
}

impl DerefMut for Logger {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.internal
  }
}
