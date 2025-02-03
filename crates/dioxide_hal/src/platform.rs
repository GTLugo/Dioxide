#[cfg(feature = "rpi5")]
pub mod raspberry_pi;

#[cfg(feature = "rpi5")]
pub use raspberry_pi::*; // re-export unwrapped

#[cfg(feature = "x86")]
pub mod x86;

// #[cfg(feature = "x86")]
// pub use x86::*; // re-export unwrapped
pub mod logger;
pub mod console;
pub mod sys;