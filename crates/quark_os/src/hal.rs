pub mod console;
mod platform_impl;

pub fn loop_forever() -> ! {
  loop {
    nop!();
  }
}
