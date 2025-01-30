use bootloader_api::BootInfo;

use self::{framebuffer::FrameBuffer, os::OS};
use crate::{eprintln, hal::sys::halt};

mod framebuffer;
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
      framebuffer: FrameBuffer::new(boot_info),
    };
    Self {
      _boot_info: boot_info,
      os,
    }
  }

  pub fn run(self) -> ! {
    if let Err(error) = self.os.run() {
      // eprintln!("FATAL | {error}");
    }
    halt()
  }
}
