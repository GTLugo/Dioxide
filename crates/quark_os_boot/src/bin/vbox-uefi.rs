#![allow(unused_attributes)]
#![allow(dead_code)]
#![feature(exit_status_error)]

mod vbox;

use std::process::ExitStatusError;

fn main() -> Result<(), ExitStatusError> {
  vbox::VirtualBox::new("quark_os_uefi", env!("UEFI_IMAGE"), vbox::Uefi::Enabled).run()
}
