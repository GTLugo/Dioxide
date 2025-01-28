#![no_std]
#![no_main]

#[quark_os::entrypoint]
fn main() {
  quark_os::OS::new().run();
}
