use core::panic::PanicInfo;

use crate::eprintln;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  eprintln!("you messed up kiddo | {info:?}");
  crate::hal::sys::halt()
}
