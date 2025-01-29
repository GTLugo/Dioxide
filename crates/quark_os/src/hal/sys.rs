use super::platform_impl;

pub fn halt() -> ! {
  loop {
    platform_impl::sys::halt();
  }
}
