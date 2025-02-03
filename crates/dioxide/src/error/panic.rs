use core::{
  panic::PanicInfo,
  sync::atomic::{AtomicBool, Ordering},
};

use dioxide_hal::platform;

use crate::eprintln;

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
  panic_guard();

  let (location, line, column) = match info.location() {
    Some(loc) => (loc.file(), loc.line(), loc.column()),
    _ => ("???", 0, 0),
  };

  let message = info.message();
  match !message.as_str().unwrap().is_empty() {
    true => eprintln!("Kernel panicked!\nLocation: `{location}` at ({line}, {column})\n`{message}`"),
    false => eprintln!("Kernel panicked!\nLocation: `{location}` at ({line}, {column})"),
  }
  
  platform::sys::halt();
}

fn panic_guard() {
  static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
  if !PANIC_IN_PROGRESS.load(Ordering::Relaxed) {
    PANIC_IN_PROGRESS.store(true, Ordering::Relaxed);
    return;
  }

  platform::sys::halt();
}
