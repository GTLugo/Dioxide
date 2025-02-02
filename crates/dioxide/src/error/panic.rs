use core::panic::PanicInfo;

use crate::{eprintln, std};

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  eprintln!("you messed up kiddo | {info:?}");
  std::sys::halt()
}
