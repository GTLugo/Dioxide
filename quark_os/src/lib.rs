#![no_std]
#![no_main]

#[macro_export]
macro_rules! nop {
  () => {
    unsafe {
      core::arch::asm!("nop");
    }
  };
}

mod hal;
mod os;
mod panic;
mod std;

pub use os::OS;

pub use quark_os_derive::entrypoint;
