use bootloader_api::BootInfo;

use self::{framebuffer::FrameBuffer, os::OS};
use crate::{eprintln, hal::sys::halt};

mod os;
mod framebuffer;

pub struct Kernel {
  boot_info: &'static mut BootInfo,
  os: OS,
}

impl Kernel {
  pub fn new(boot_info: &'static mut BootInfo) -> Self {
    let os = OS {
      framebuffer: FrameBuffer::new(boot_info),
    };
    Self { boot_info, os }
  }

  pub fn run(self) -> ! {
    if let Err(error) = self.os.run() {
      // eprintln!("FATAL | {error}");
    }
    halt()
  }
}
