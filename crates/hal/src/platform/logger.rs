use core::ops::{Deref, DerefMut};

use bootloader_api::info::FrameBufferInfo;
use dioxide_std::cheat_sync::OnceLock;

use crate::platform::*;

pub trait PlatformLogger: Send + Sync {
  type Internal;

  fn new(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) -> Self
  where
    Self: Sized;

  fn internal(&self) -> &Self::Internal;
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn logger() -> &'static Logger {
  LOGGER.get().unwrap()
}

pub fn init(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) {
  let logger = LOGGER.get_or_init(|| Logger::new(framebuffer, framebuffer_info));
  log::set_logger(logger.internal()).expect("Logger already set");
  log::set_max_level(log::LevelFilter::Trace);
  log::info!("Logger initialized");
}

pub struct Logger {
  #[cfg(feature = "x86")]
  internal: x86::logger::X86Logger,
}

impl Logger {
  fn new(framebuffer: &'static mut [u8], framebuffer_info: FrameBufferInfo) -> Self {
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
