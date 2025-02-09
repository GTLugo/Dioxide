use crate::platform::*;

#[inline]
pub fn halt() -> ! {
  #[cfg(feature = "x86")]
  loop {
    x86::sys::halt();
  }
}
