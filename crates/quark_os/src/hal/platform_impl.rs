#[cfg(any(feature = "rpi5"))]
mod raspberry_pi;

#[cfg(any(feature = "rpi5"))]
pub use raspberry_pi::*; // re-export unwrapped

#[cfg(any(feature = "x86"))]
mod x86;

#[cfg(any(feature = "x86"))]
pub use x86::*; // re-export unwrapped