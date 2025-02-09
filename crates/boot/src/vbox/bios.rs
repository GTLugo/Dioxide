#![allow(unused_attributes)]
#![allow(dead_code)]
#![feature(exit_status_error)]

mod vbox;

use std::process::ExitStatusError;

fn main() -> Result<(), ExitStatusError> {
  vbox::VirtualBox::new("dioxide_bios", env!("BIOS_IMAGE"), vbox::Uefi::Disabled).run()
}
