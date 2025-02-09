#![allow(unused_attributes)]
#![allow(dead_code)]
#![feature(exit_status_error)]

use std::{
  fmt::Display,
  path::PathBuf,
  process::{Command, ExitStatusError, Stdio},
};

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

pub struct VirtualBox {
  name: String,
  image: PathBuf,
  vdi: PathBuf,
  _uefi: Uefi,
}

// TODO: Make better error handlers

impl VirtualBox {
  pub fn new(name: impl Into<String>, image: impl Into<PathBuf>, uefi: Uefi) -> Self {
    let image = image.into();
    let current_exe = std::env::current_exe().unwrap();
    let vdi = current_exe.with_file_name(format!("{uefi}.vdi"));
    Self {
      name: name.into(),
      image,
      vdi,
      _uefi: uefi,
    }
  }

  pub fn run(&self) -> Result<(), ExitStatusError> {
    self.stop();
    self.remove_old_image();
    self.set_new_image()?;
    self.start()
  }

  fn stop(&self) {
    let mut vm = Command::new("VBoxManage");
    vm.arg("startvm").arg(&self.name).arg("--type").arg("emergencystop");
    vm.stderr(Stdio::null());
    let _ = vm.status().unwrap();
  }

  fn remove_old_image(&self) {
    let mut vm = Command::new("VBoxManage");
    vm.arg("storageattach")
      .arg(&self.name)
      .arg("--storagectl")
      .arg("IDE")
      .arg("--port")
      .arg("0")
      .arg("--device")
      .arg("0")
      .arg("--type")
      .arg("hdd")
      .arg("--medium")
      .arg("none");
    vm.stderr(Stdio::null());
    let exit_status = vm.status().unwrap();

    if !exit_status.success() {
      eprintln!("detach error");
      // process::exit(exit_status.code().unwrap_or(-1));
    }

    let mut vm = Command::new("VBoxManage");
    vm.arg("closemedium").arg("disk").arg(&self.vdi).arg("--delete");
    vm.stderr(Stdio::null());
    let exit_status = vm.status().unwrap();

    if !exit_status.success() {
      eprintln!("closemedium error");
      // process::exit(exit_status.code().unwrap_or(-1));
    }

    // exit_status.exit_ok()

    // let _ = remove_file(&vdi_dir);
  }

  fn set_new_image(&self) -> Result<(), ExitStatusError> {
    let mut vm = Command::new("VBoxManage");
    vm.arg("convertfromraw")
      .arg(&self.image)
      .arg(&self.vdi)
      .arg("--format")
      .arg("VDI");
    vm.stderr(Stdio::null());
    let exit_status = vm.status().unwrap();

    if !exit_status.success() {
      eprintln!("convertfromraw error");
      // process::exit(exit_status.code().unwrap_or(-1));
    }

    let mut vm = Command::new("VBoxManage");
    vm.arg("modifymedium").arg(&self.vdi).arg("--resize").arg("2048");
    vm.stderr(Stdio::null());
    let exit_status = vm.status().unwrap();

    if !exit_status.success() {
      eprintln!("modifyhd error");
      // process::exit(exit_status.code().unwrap_or(-1));
    }

    let mut vm = Command::new("VBoxManage");
    vm.arg("storageattach")
      .arg(&self.name)
      .arg("--storagectl")
      .arg("IDE")
      .arg("--port")
      .arg("0")
      .arg("--device")
      .arg("0")
      .arg("--type")
      .arg("hdd")
      .arg("--medium")
      .arg(&self.vdi);
    vm.stderr(Stdio::null());
    let exit_status = vm.status().unwrap();

    exit_status.exit_ok()
  }

  fn start(&self) -> Result<(), ExitStatusError> {
    let mut vm = Command::new("VBoxManage");
    vm.arg("startvm").arg(&self.name);
    vm.stderr(Stdio::null());
    let exit_status = vm.status().unwrap();

    exit_status.exit_ok()
  }
}

fn main() {}
