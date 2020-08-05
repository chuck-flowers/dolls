#![warn(clippy::all)]
#![feature(const_generics)]

#[cfg(feature = "dll")]
pub mod dll;
#[cfg(feature = "elf")]
pub mod elf;
pub mod errors;
mod parsing;
mod shared;

pub use self::parsing::Parse;
