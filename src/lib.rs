#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

//! Manage your opencl3 devices and platforms
//!
//! # Features
//! - [serde] support for (de)serialization
//! - [display] provides a CLI display

mod clinfo;
#[cfg(feature = "display")]
mod display;
mod error;
#[cfg(feature = "serde")]
mod storage;

pub use clinfo::*;
#[cfg(feature = "display")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "display")))]
pub use display::*;
pub use error::*;
#[cfg(feature = "serde")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "serde")))]
pub use storage::*;
