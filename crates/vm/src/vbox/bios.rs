use std::process::ExitStatusError;

use super::VirtualBox;
use crate::vm::Uefi;

pub fn run() -> Result<(), ExitStatusError> {
  VirtualBox::new("dioxide_bios", env!("BIOS_IMAGE"), Uefi::Disabled).run()
}
