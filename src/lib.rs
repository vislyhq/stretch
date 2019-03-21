#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]
#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod geometry;
pub mod layout;
pub mod number;
pub mod result;
pub mod style;

mod algo;
pub use crate::algo::{compute, Database};
