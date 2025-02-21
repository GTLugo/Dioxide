///
/// THIS IS AI-GENERATED! DO NOT USE!!!
///
/// NOT FOR ACTUAL USE, ONLY FOR BRAINSTORMING AND TESTING.
/// ANY CODE HERE SHOULD BE THOROUGHLY SCRUTINIZED AND
/// TESTED BEFORE TRUSTING!!!
/// 
/// This must be swapped out after doing more research into
/// shared locks and atomics.
///
use core::{
  cell::UnsafeCell,
  hint::spin_loop,
  mem::MaybeUninit,
  sync::atomic::{AtomicU8, Ordering},
};

/// Our state for the OnceLock. We have three variants:
/// - `Uninit`: The value hasn’t been initialized yet.
/// - `Initing`: A thread is currently initializing it.
/// - `Init`: The value is fully initialized.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
  Uninitialized = 0,
  Initializing = 1,
  Ready = 2, // Different name style to make it obvious in code
}

impl State {
  const fn as_u8(self) -> u8 {
    self as u8
  }
}

pub struct OnceLock<T> {
  state: AtomicU8,
  value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Default for OnceLock<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> OnceLock<T> {
  /// Create a new, uninitialized OnceLock.
  pub const fn new() -> Self {
    Self {
      state: AtomicU8::new(State::Uninitialized.as_u8()),
      value: UnsafeCell::new(MaybeUninit::uninit()),
    }
  }

  #[inline]
  fn is_initialized(&self) -> bool {
    self.state.load(Ordering::Acquire) == State::Ready.as_u8()
  }

  /// Returns a shared reference to the value if it is initialized,
  /// or `None` otherwise.
  pub fn get(&self) -> Option<&T> {
    (self.is_initialized()).then(|| {
      // SAFETY: The value is guaranteed to be initialized.
      unsafe { &*((*self.value.get()).as_ptr()) }
    })
  }

  /// Returns a mutable reference to the value if initialized
  pub fn get_mut(&self) -> Option<&mut T> {
    // With exclusive access, we can check the state without worrying about races.
    (self.is_initialized()).then(|| {
      // SAFETY: We have a mutable reference and the value is fully initialized.
      unsafe { &mut *((*self.value.get()).as_mut_ptr()) }
    })
  }

  /// Attempts to set the value. If the OnceLock was uninitialized, it becomes
  /// initialized and returns `Ok(())`. If it was already initialized or in the
  /// process of initializing, returns back the provided value in `Err(value)`.
  pub fn set(&self, value: T) -> Result<(), T> {
    // Try to atomically move from Uninit -> Initing.
    if self
      .state
      .compare_exchange(State::Uninitialized.as_u8(), State::Initializing.as_u8(), Ordering::Acquire, Ordering::Relaxed)
      .is_ok()
    {
      // SAFETY: We have exclusive access to write the value.
      unsafe {
        (*self.value.get()).as_mut_ptr().write(value);
      }
      // Publish the initialization.
      self.state.store(State::Ready.as_u8(), Ordering::Release);
      Ok(())
    } else {
      Err(value)
    }
  }

  /// Returns a reference to the value, initializing it with the provided closure
  /// if necessary. If another thread is initializing, this spins until complete.
  pub fn get_or_init<F>(&self, init: F) -> &T
  where
    F: FnOnce() -> T,
  {
    // Fast path: already initialized.
    if self.state.load(Ordering::Acquire) == State::Ready.as_u8() {
      return unsafe { &*((*self.value.get()).as_ptr()) };
    }

    // Attempt to claim initialization.
    let initialize = self
      .state
      .compare_exchange(State::Uninitialized.as_u8(), State::Initializing.as_u8(), Ordering::Acquire, Ordering::Relaxed)
      .is_ok();
    match initialize {
      true => {
        let value = init();
        unsafe {
          (*self.value.get()).as_mut_ptr().write(value);
        }
        self.state.store(State::Ready.as_u8(), Ordering::Release);
      }
      false => {
        // Another thread is initializing; spin until done.
        while self.state.load(Ordering::Acquire) != State::Ready.as_u8() {
          spin_loop();
        }
      }
    }
    unsafe { &*((*self.value.get()).as_ptr()) }
  }
}

// Since we’re using interior mutability, we have to assert that OnceLock<T> is Sync/Send
// appropriately.
unsafe impl<T: Sync> Sync for OnceLock<T> {}
unsafe impl<T: Send> Send for OnceLock<T> {}

impl<T> Drop for OnceLock<T> {
  fn drop(&mut self) {
    // Only drop the inner value if it was successfully initialized.
    if self.state.load(Ordering::Acquire) == State::Ready.as_u8() {
      unsafe {
        core::ptr::drop_in_place((*self.value.get()).as_mut_ptr());
      }
    }
  }
}
