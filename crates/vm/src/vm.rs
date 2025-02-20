use std::fmt::Display;

pub enum Uefi {
  Enabled,
  Disabled,
}

impl Display for Uefi {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Self::Enabled => "uefi",
      Self::Disabled => "bios",
    })
  }
}

pub enum VirtualMachine {
  VirtualBox,
  Qemu,
}

impl Display for VirtualMachine {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Self::VirtualBox => "vbox",
      Self::Qemu => "qemu",
    })
  }
}

