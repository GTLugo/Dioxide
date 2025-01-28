use core::panic::PanicInfo;

use crate::hal::loop_forever;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
  loop_forever()
}
