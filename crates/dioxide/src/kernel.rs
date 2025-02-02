use bootloader_api::BootInfo;

use self::{canvas::Canvas, os::OS};
use crate::std::sys::halt;

mod canvas;
mod os;

///
/// This pretty much just sets up the OS. Wraps and converts FFI boot objects.
///
pub struct Kernel {
  _boot_info: &'static mut BootInfo,
  os: OS,
}

impl Kernel {
  pub fn new(boot_info: &'static mut BootInfo) -> Self {
    let os = OS {
      canvas: Canvas::new(boot_info),
    };
    Self {
      _boot_info: boot_info,
      os,
    }
  }

  pub fn run(self) -> ! {
    if let Err(error) = self.os.run() {
      panic!("FATAL | {error}");
    }
    halt()
  }
}
