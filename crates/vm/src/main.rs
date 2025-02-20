#![allow(unused_attributes)]
#![allow(dead_code)]
#![feature(exit_status_error)]

mod vm;

#[cfg(feature = "vbox")]
mod vbox;

#[cfg(feature = "qemu")]
mod qemu;

use std::{env, fs, process::ExitStatusError};

fn main() -> Result<(), ExitStatusError> {
  #[allow(unreachable_code)]
  {
    #[cfg(feature = "vbox")]
    return vbox::run();
    #[cfg(feature = "qemu")]
    return qemu::run();
    run()
  }
}

fn run() -> Result<(), ExitStatusError> {
  let current_exe = env::current_exe().unwrap();
  let uefi_target = current_exe.with_file_name("uefi.img");
  let bios_target = current_exe.with_file_name("bios.img");

  fs::copy(env!("UEFI_IMAGE"), &uefi_target).unwrap();
  fs::copy(env!("BIOS_IMAGE"), &bios_target).unwrap();

  println!("UEFI disk image at {}", uefi_target.display());
  println!("BIOS disk image at {}", bios_target.display());

  Ok(())
}
