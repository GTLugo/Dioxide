pub trait Diagnostics {
  #[allow(unused)]
  fn chars_written(&self) -> usize;
}

pub trait PlatformConsole: core::fmt::Write + Diagnostics {
  fn new() -> Self;
}

// pub fn console() -> &'static impl PlatformConsole {
//   #[cfg(feature = "x86")]
//   x86::console::console()
// }
