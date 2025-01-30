#![no_std]
#![no_main]

// #[quark_os::entrypoint]
bootloader_api::entry_point!(main);

fn main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
  dioxide::Kernel::new(boot_info).run()
}
