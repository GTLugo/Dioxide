use std::path::PathBuf;

use bootloader::DiskImageBuilder;

fn main() {
  // println!("cargo:rerun-if-changed=./linker.ld");
  let entry_path = std::env::var("CARGO_BIN_FILE_DIOXIDE").unwrap();
  eprintln!("{entry_path:?}");
  let disk_builder = DiskImageBuilder::new(PathBuf::from(entry_path));

  let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let uefi_path = out_dir.join("dioxide-uefi.img");
  let bios_path = out_dir.join("dioxide-bios.img");

  disk_builder.create_uefi_image(&uefi_path).unwrap();
  disk_builder.create_bios_image(&bios_path).unwrap();

  // convert to vdi
  

  println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
  println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}
