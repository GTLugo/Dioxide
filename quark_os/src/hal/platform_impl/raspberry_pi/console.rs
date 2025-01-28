use crate::{
  hal::console::{Console, Diagnostics},
  std::*,
};

use self::sync::{LazyLock, Mutex, MutexGuard};

pub struct PiConsole {
  chars_written: usize,
}

impl PiConsole {
  fn new() -> Self {
    Self { chars_written: 0 }
  }
}

impl Console for PiConsole {}

impl core::fmt::Write for PiConsole {
  fn write_char(&mut self, c: char) -> core::fmt::Result {
    unsafe {
      core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
    }

    self.chars_written += 1;

    Ok(())
  }

  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    for c in s.chars() {
      if c == '\n' {
        self.write_char('\r')?;
      }

      self.write_char(c)?;
    }

    Ok(())
  }
}

impl Diagnostics for PiConsole {
  fn chars_written(&self) -> usize {
    self.chars_written
  }
}

pub fn console() -> MutexGuard<'static, impl Console> {
  static CONSOLE: LazyLock<Mutex<PiConsole>> =
    LazyLock::new(|| Mutex::new(PiConsole::new()));
  CONSOLE.lock()
}
