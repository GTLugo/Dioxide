#![feature(exit_status_error)]

use std::{
  env,
  path::PathBuf,
  process::{Command, ExitStatusError, Stdio},
};

const NAME: &str = "quark_os_uefi";

fn main() -> Result<(), ExitStatusError> {
  let img_dir = PathBuf::from(env!("UEFI_IMAGE"));
  let current_exe = env::current_exe().unwrap();
  let vdi_dir = current_exe.with_file_name("uefi.vdi");

  stop();
  remove_old_image(&vdi_dir)?;
  set_new_image(&img_dir, &vdi_dir)?;
  start()
}

fn stop() {
  let mut vm = Command::new("VBoxManage");
  vm.arg("startvm").arg(NAME).arg("--type").arg("emergencystop");
  vm.stderr(Stdio::null());
  let _ = vm.status().unwrap();
}

fn remove_old_image(vdi_dir: &PathBuf) -> Result<(), ExitStatusError> {
  let mut vm = Command::new("VBoxManage");
  vm.arg("storageattach")
    .arg(NAME)
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
  vm.arg("closemedium").arg("disk").arg(vdi_dir).arg("--delete");
  vm.stderr(Stdio::null());
  let exit_status = vm.status().unwrap();

  exit_status.exit_ok()

  // let _ = remove_file(&vdi_dir);
}

fn set_new_image(img_dir: &PathBuf, vdi_dir: &PathBuf) -> Result<(), ExitStatusError> {
  let mut vm = Command::new("VBoxManage");
  vm.arg("convertfromraw")
    .arg(img_dir)
    .arg(vdi_dir)
    .arg("--format")
    .arg("VDI");
  vm.stderr(Stdio::null());
  let exit_status = vm.status().unwrap();

  if !exit_status.success() {
    eprintln!("convertfromraw error");
    // process::exit(exit_status.code().unwrap_or(-1));
  }

  let mut vm = Command::new("VBoxManage");
  vm.arg("modifymedium").arg(vdi_dir).arg("--resize").arg("2048");
  vm.stderr(Stdio::null());
  let exit_status = vm.status().unwrap();

  if !exit_status.success() {
    eprintln!("modifyhd error");
    // process::exit(exit_status.code().unwrap_or(-1));
  }

  let mut vm = Command::new("VBoxManage");
  vm.arg("storageattach")
    .arg(NAME)
    .arg("--storagectl")
    .arg("IDE")
    .arg("--port")
    .arg("0")
    .arg("--device")
    .arg("0")
    .arg("--type")
    .arg("hdd")
    .arg("--medium")
    .arg(vdi_dir);
  vm.stderr(Stdio::null());
  let exit_status = vm.status().unwrap();

  exit_status.exit_ok()
}

fn start() -> Result<(), ExitStatusError> {
  let mut vm = Command::new("VBoxManage");
  vm.arg("startvm").arg(NAME);
  vm.stderr(Stdio::null());
  let exit_status = vm.status().unwrap();

  exit_status.exit_ok()
}
