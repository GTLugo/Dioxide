use bootloader_api::BootInfo;
use conquer_once::spin::OnceCell;
use dioxide_hal::platform::{
  self,
  logger::{Logger, PlatformLogger},
};

use self::{framebuffer::FrameBuffer, os::OS};

pub(crate) mod framebuffer;
mod os;

static LOGGER: OnceCell<Logger> = OnceCell::uninit();

///
/// This pretty much just sets up the OS. Wraps and converts FFI boot objects.
///
pub struct Kernel {
  os: OS,
}

impl Kernel {
  pub fn new(boot_info: &'static mut BootInfo) -> Option<Self> {
    let mut framebuffer = FrameBuffer::<'static>::new(boot_info)?;
    Self::setup_logger(&mut framebuffer);

    let os = OS { framebuffer };

    Some(Self { os })
  }

  fn setup_logger(framebuffer: &mut FrameBuffer<'static>) {
    let logger = LOGGER.get_or_init(|| Logger::new(unsafe { framebuffer.buffer() }, *framebuffer.info()));
    log::set_logger(logger.internal()).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Logger initialized");
  }

  pub fn run(self) -> ! {
    if let Err(error) = self.os.run() {
      panic!("{error}");
    }
    platform::sys::halt();
  }
}
