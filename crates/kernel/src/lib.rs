#![no_std]
#![no_main]
#![feature(format_args_nl)]

#[macro_export]
macro_rules! nop {
  () => {
    unsafe {
      core::arch::asm!("nop");
    }
  };
}

pub mod kernel;

mod error;

// pub use dioxide_derive::entrypoint;

pub use crate::kernel::Kernel;
