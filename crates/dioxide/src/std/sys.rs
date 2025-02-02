use crate::hal::platform_impl;

pub fn halt() -> ! {
  loop {
    platform_impl::sys::halt();
  }
}
