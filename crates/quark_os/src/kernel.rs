use bootloader_api::BootInfo;

use self::os::OS;
use crate::{eprintln, hal::sys::halt};

mod os;

pub struct Kernel {
  boot_info: &'static mut BootInfo,
  os: OS,
}

impl Kernel {
  pub fn new(boot_info: &'static mut BootInfo) -> Self {
    let os = OS {
      framebuffer: boot_info.framebuffer.take(),
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
