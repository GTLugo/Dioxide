use core::fmt::Display;

use thiserror::Error;

mod panic;

#[derive(Debug)]
pub struct ErrorCode(i32);

impl Display for ErrorCode {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Error, Debug)]
pub enum OsError {
  #[error("OS error | unknown error")]
  Unknown,
  #[error("OS error | out of bounds access at index `{0}`")]
  OutOfBounds(usize),
}

// #[allow(unused)]
// impl OsError {
//   pub fn new(code: i32) -> Self {
//     Self { code: ErrorCode(code) }
//   }
// }
