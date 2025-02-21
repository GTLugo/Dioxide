use dioxide_std::cheat_sync::OnceLock;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::platform;

static IDT: OnceLock<InterruptDescriptorTable> = OnceLock::new();

pub fn idt() -> &'static mut InterruptDescriptorTable {
  let _ = IDT.get_or_init(InterruptDescriptorTable::new);
  IDT.get_mut().unwrap()
}

pub fn init_idt() {
  idt().breakpoint.set_handler_fn(breakpoint_handler);
  idt().double_fault.set_handler_fn(double_fault_handler);
  idt().load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
  log::error!("CPU Exception!\n{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
  log::error!("CPU Double Fault! ({error_code})\n{stack_frame:#?}");
  platform::sys::halt()
}

pub fn breakpoint() {
  x86_64::instructions::interrupts::int3();
}