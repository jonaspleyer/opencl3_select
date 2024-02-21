#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

//! Manage your opencl3 devices and platforms
//!
//! # Features
//! - [serde] support for (de)serialization
//! - [ratatui] provides a CLI display

mod clinfo;
#[cfg(feature = "ratatui")]
mod display;
mod error;
mod priority;
#[cfg(feature = "serde")]
mod storage;

pub use clinfo::*;
#[cfg(feature = "ratatui")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "ratatui")))]
pub use display::*;
pub use error::*;
pub use priority::*;
#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
pub use storage::*;
