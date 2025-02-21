pub fn init_idt() {
  #[cfg(feature = "x86")]
  super::x86::interrupts::init_idt();
}

pub fn breakpoint() {
  
}