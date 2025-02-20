use std::{
  env,
  process::{self, Command},
};

pub fn run() {
  let mut qemu = Command::new("qemu-system-x86_64");
  qemu
    .arg("-drive")
    .arg(format!("format=raw,file={}", env!("BIOS_IMAGE")))
    .arg("-no-reboot");
  let exit_status = qemu.status().unwrap();
  process::exit(exit_status.code().unwrap_or(-1));
}
