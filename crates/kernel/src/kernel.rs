use bootloader_api::BootInfo;
use dioxide_hal::platform::{self};
use dioxide_std::eprintln;

use self::{framebuffer::FrameBuffer, os::OS};

pub(crate) mod framebuffer;
mod os;

///
/// This pretty much just sets up the OS. Wraps and converts FFI boot objects.
///
pub struct Kernel {
  os: OS,
}

impl Kernel {
  pub fn new(boot_info: &'static mut BootInfo) -> Option<Self> {
    let mut framebuffer = FrameBuffer::<'static>::new(boot_info)?;
    platform::logger::init(unsafe { framebuffer.buffer() }, *framebuffer.info());

    log::info!("Setting up Dioxide OS");
    let os = OS { framebuffer };

    Some(Self { os })
  }

  pub fn run(self) -> ! {
    log::info!("Entering Dioxide OS");
    let result = self.os.run();
    log::info!("Exiting Dioxide OS");
    if let Err(error) = result {
      eprintln!("{error}");
    }
    platform::sys::halt();
  }
}
