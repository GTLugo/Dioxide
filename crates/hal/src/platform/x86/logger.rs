use bootloader_api::info::FrameBufferInfo;
use bootloader_x86_64_common::logger::LockedLogger;

use crate::platform::logger::PlatformLogger;

pub struct X86Logger {
  internal: LockedLogger,
}

impl PlatformLogger for X86Logger {
  type Internal = LockedLogger;

  fn new(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) -> Self {
    Self {
      internal: LockedLogger::new(framebuffer, framebuffer_info, true, false),
    }
  }

  fn internal(&self) -> &Self::Internal {
    &self.internal
  }
}
