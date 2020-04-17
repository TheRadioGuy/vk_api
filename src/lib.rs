#![warn(clippy::all)]
#[macro_use]
extern crate json;

pub mod params;
pub mod vk;
pub use vk::VK;
pub use params::Params;
