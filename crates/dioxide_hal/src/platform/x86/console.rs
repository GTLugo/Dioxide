use crate::platform::console::{Diagnostics, PlatformConsole};

pub struct X86Console {
  chars_written: usize,
}

impl PlatformConsole for X86Console {
  fn new() -> Self {
    Self { chars_written: 0 }
  }
}

impl core::fmt::Write for X86Console {
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

impl Diagnostics for X86Console {
  fn chars_written(&self) -> usize {
    self.chars_written
  }
}

// pub fn console() -> &'static X86Console {
//   static CONSOLE: OnceCell<X86Console> = OnceCell::uninit();
//   CONSOLE.get_or_init(X86Console::new)
// }
