#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

//! The 'nessak' Rust crate is the implementation of the experimental hashing algorithm named Nessak
//! and the implementation of the experimental hashing algorithm structure named Tundra.
//!
//! More information on the algorithm: <https://github.com/Zffu/Nessak>
//!
//! This crate is no_std compatible and requires the alloc crate.
//!

extern crate alloc;

/// The Tundra implementation.
/// Also contains a Nessak implementation that uses the Tundra implementation.
#[cfg(feature = "tundra")]
pub mod tundra;

#[cfg(feature = "standalone")]
pub mod standalone;

/// Implementations of the Nessak inner functions.
#[cfg(feature = "nessak_impls")]
pub mod impls;

/// The configuration presets for the const versions of the implementations.
/// There is one preset per "recognized" Nessak standard with the ability of creating your own.
#[cfg(feature = "const")]
pub mod preset;

pub(crate) mod utils;
