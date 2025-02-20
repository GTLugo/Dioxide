#[cfg(feature = "uefi")]
pub mod uefi;
#[cfg(feature = "uefi")]
pub use uefi::*;

#[cfg(feature = "bios")]
pub mod bios;
#[cfg(feature = "bios")]
pub use bios::*;