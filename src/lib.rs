#![warn(clippy::all)]
#[macro_use]
extern crate json;

pub mod params;
pub mod vk;
pub use params::Params;
pub use vk::VK;
