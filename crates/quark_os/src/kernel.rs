use self::os::OS;
use crate::{eprintln, hal::sys::halt};

mod os;

pub struct Kernel {
  os: OS,
}

impl Kernel {
  pub fn new(_boot_info: &'static mut bootloader_api::BootInfo) -> Self {
    let os = OS::default();
    Self { os }
  }

  pub fn run(self) -> ! {
    if let Err(error) = self.os.run() {
      // eprintln!("FATAL | {error}");
    }
    halt()
  }
}
