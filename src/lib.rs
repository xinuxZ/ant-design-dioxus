//! # Ant Design Dioxus
//!
//! Ant Design implementation for Dioxus framework.
//!
//! This library provides a set of UI components following Ant Design guidelines.

#![allow(non_snake_case)]
#![allow(clippy::needless_doctest_main)]

pub mod components;
pub mod config_provider;
pub mod hooks;
pub mod locale;
pub mod prelude;
pub mod shared;
pub mod theme;
pub mod utils;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// pub use components::*;
pub use config_provider::GlobalConfigContext;
// pub use prelude::*;
