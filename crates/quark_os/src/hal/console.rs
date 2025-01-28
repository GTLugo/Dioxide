use crate::std::sync::MutexGuard;

use super::platform_impl;

pub trait Diagnostics {
  #[allow(unused)]
  fn chars_written(&self) -> usize;
}

pub trait Console: core::fmt::Write + Diagnostics {}

pub fn console() -> MutexGuard<'static, impl Console> {
  platform_impl::console::console()
}
