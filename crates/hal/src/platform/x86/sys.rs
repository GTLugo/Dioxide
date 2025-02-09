#[inline]
pub fn halt() {
  unsafe { core::arch::asm!("hlt") };
}
