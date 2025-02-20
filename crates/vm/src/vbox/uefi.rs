use std::process::ExitStatusError;

use super::VirtualBox;
use crate::vm::Uefi;

pub fn run() -> Result<(), ExitStatusError> {
  VirtualBox::new("dioxide_uefi", env!("UEFI_IMAGE"), Uefi::Enabled).run()
}
