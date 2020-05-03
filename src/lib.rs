//! Rust crate for interacting with API
//! # Quick example:
//! ```rust
//! let mut params = vk_api::Params::new();
//! let params = params.add("group_id", "142102660").add("fields", "bdate"); // add params
//! let mut vk_api = vk_api::VK::new("5.103", "ru"); // 5.103 is api version
//! vk_api.set_access_token("ACCESS_TOKEN".to_string()); // Access token is your token
//! let response = vk_api.request("groups.getMembers", params).unwrap(); // call groups.getMembers method with our parametres
//!     for user in response["response"]["items"].members() {
//!        println!("Имя: {}, Фамилия: {}, Дата рождения: {}", user["first_name"], user["last_name"], user["bdate"]); // Print all users information
//!    }
//! ```
//! Other examples are [hLongpollere](https://github.com/DuckerMan/vk_api/tree/master/examples)
#![warn(clippy::all)]
#[macro_use]
extern crate json;
extern crate futures;

pub mod longpoll;
pub mod params;
pub mod vk;
pub mod types {
    pub mod destination;
    pub mod file;
}
mod utils;

pub use longpoll::Longpoll;
pub use params::Params;
pub use types::destination::Destination;
pub use types::file::File;
pub use vk::VK;
