#![allow(unused)]

use core::{
  cell::{LazyCell, UnsafeCell},
  hint::spin_loop,
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  sync::atomic::{AtomicBool, AtomicU64},
};

pub struct Mutex<T> {
  data: UnsafeCell<T>,
  is_acquired: AtomicBool,
}

impl<T> Mutex<T> {
  pub const fn new(value: T) -> Self {
    Self {
      data: UnsafeCell::new(value),
      is_acquired: AtomicBool::new(false),
    }
  }

  pub fn lock<'a>(&'a self) -> MutexGuard<'a, T> {
    while !self
      .is_acquired
      .swap(true, core::sync::atomic::Ordering::AcqRel)
    {
      spin_loop();
    }
    MutexGuard { mutex: self }
  }

  pub fn unlock(&self) {
    self
      .is_acquired
      .store(false, core::sync::atomic::Ordering::Release);
  }
}

pub struct MutexGuard<'a, T> {
  mutex: &'a Mutex<T>,
}

impl<T> Drop for MutexGuard<'_, T> {
  fn drop(&mut self) {
    self.mutex.unlock()
  }
}

impl<T> Deref for MutexGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.mutex.data.get() }
  }
}

impl<T> DerefMut for MutexGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.mutex.data.get() }
  }
}

unsafe impl<T> Send for Mutex<T> where T: Send {}
unsafe impl<T> Sync for Mutex<T> where T: Send {}
unsafe impl<T> Send for MutexGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for MutexGuard<'_, T> where T: Send + Sync {}

enum Data<T, F> {
  Uninitialized(Option<F>),
  Initialized(T),
}

pub struct LazyLock<T, F = fn() -> T> {
  once: Once,
  data: UnsafeCell<Data<T, F>>,
}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
  pub const fn new(f: F) -> Self {
    Self {
      once: Once::new(),
      data: UnsafeCell::new(Data::Uninitialized(Some(f))),
    }
  }

  pub fn force(&self) -> &T {
    self.once.call_once(|| {
      let data = unsafe { &mut *self.data.get() };
      let f = match data {
        Data::Uninitialized(f) => f.take(),
        Data::Initialized(_) => unreachable!("invalid state for LazyLock"),
      };
      let value = f.unwrap()();
      *data = Data::Initialized(value);
    });

    let data = unsafe { &mut *self.data.get() };
    match data {
      Data::Uninitialized(_) => unreachable!("invalid state for LazyLock"),
      Data::Initialized(v) => v,
    }
  }
}

impl<T, F> LazyLock<T, F> {
  fn get(&self) -> Option<&T> {
    let data = unsafe { &mut *self.data.get() };

    match data {
      Data::Uninitialized(_) => None,
      Data::Initialized(v) => Some(v),
    }
  }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    Self::force(self)
  }
}

impl<T: Default> Default for LazyLock<T> {
  fn default() -> Self {
    Self::new(T::default)
  }
}

unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}

pub struct Once {
  is_initialized: AtomicBool,
  mutex: Mutex<()>,
}

impl Once {
  pub const fn new() -> Self {
    Once {
      is_initialized: AtomicBool::new(false),
      mutex: Mutex::new(()),
    }
  }

  pub fn call_once(&self, mut init: impl FnMut()) {
    if !self
      .is_initialized
      .load(core::sync::atomic::Ordering::Acquire)
    {
      let _ = self.mutex.lock();

      if !self
        .is_initialized
        .load(core::sync::atomic::Ordering::Relaxed)
      {
        init();
        self
          .is_initialized
          .store(true, core::sync::atomic::Ordering::Release);
      }
    }
  }
}
