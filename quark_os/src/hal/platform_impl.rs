#[cfg(any(feature = "rpi5"))]
mod raspberry_pi;

#[cfg(any(feature = "rpi5"))]
pub use raspberry_pi::*; // re-export unwrapped