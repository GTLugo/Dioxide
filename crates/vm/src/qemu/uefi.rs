use std::{
  env,
  process::{self, Command},
};

use ovmf_prebuilt::{Arch, FileType, Prebuilt, Source};

pub fn run() {
  let prebuilt = Prebuilt::fetch(Source::LATEST, "target/ovmf").expect("failed to update prebuilt");
  let mut qemu = Command::new("qemu-system-x86_64");
  qemu
    .arg("-drive")
    .arg(format!("format=raw,file={}", env!("UEFI_IMAGE")))
    .arg("-drive")
    .arg(format!(
      "if=pflash,format=raw,readonly=on,file={}",
      prebuilt.get_file(Arch::X64, FileType::Code).to_str().unwrap()
    ))
    .arg("-drive")
    .arg(format!(
      "if=pflash,format=raw,file={}",
      prebuilt.get_file(Arch::X64, FileType::Vars).to_str().unwrap()
    ))
    .arg("-no-reboot");
  let exit_status = qemu.status().unwrap();
  process::exit(exit_status.code().unwrap_or(-1));
}
