#![warn(clippy::all, rust_2018_idioms)]

pub mod gui;
pub mod model;
pub mod websocket_client;
pub use gui::GUI;
