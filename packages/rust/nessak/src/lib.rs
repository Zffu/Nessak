#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![no_std]

//! The 'nessak' Rust crate is the implementation of the experimental hashing algorithm named Nessak
//! and the implementation of the experimental hashing algorithm structure named Tundra.
//!
//! More information on the algorithm: <https://github.com/Zffu/Nessak>
//!
//! This crate is no_std compatible and requires the alloc crate.
//!
//! ## Examples
//! ### Nessak with bundled Tundra structure
//! ```
//! # #[cfg(all(not(feature = "const"), feature = "tundra-nessak"))] {
//! use nessak::tundra::nessak::NessakTundraImplementation;
//!
//! let hash = NessakTundraImplementation::k256_256(&[]); // Use a common standard
//!  # }
//! ```
//!
//! Or with the `const` variant:
//! ```
//! # #[cfg(all(feature = "const", feature = "tundra-nessak"))] {
//! use nessak::presets::{NessakK256_256, TundraPreset};
//! use nessak::tundra::nessak::NessakTundraImplementation;
//!
//! let hash = NessakTundraImplementation::produce_hash::<NessakK256_256>(&[]); // Use a common standard
//!
//! pub struct MyPreset;
//! impl TundraPreset for MyPreset {
//!		const DIGEST_SIZE: usize = 128;
//!		const LANE_SIZE: usize = 128;
//!		const COMPRESSION_ROUNDS: usize = 64;
//!		const DESCENT_COMPRESSION_ROUNDS: usize = 16;
//!		const INNER_PERMUTATION_ROUNDS: usize = 24;
//!		const OUTER_PERMUTATION_ROUNDS: usize = 16;
//!		const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize = 2;
//! } // Or make your own!
//!
//! let hash = NessakTundraImplementation::produce_hash::<MyPreset>(&[]); // And use it like any other preset
//! }
//! ```
//!
//! ### Standalone Nessak
//! ```
//! # #[cfg(all(not(feature = "const"), feature = "standalone"))] {
//! use nessak::standalone::Nessak;
//!
//! let hash = Nessak::k256_256(&[]); // Use a common standard
//!  # }
//! ```
//!
//! Or with the `const` variant:
//! ```
//! # #[cfg(all(feature = "const", feature = "standalone"))] {
//! use nessak::presets::{NessakK256_256, TundraPreset};
//! use nessak::tundra::nessak::Nessak;
//!
//! let hash = Nessak::produce_hash::<NessakK256_256>(&[]); // Use a common standard
//!
//! pub struct MyPreset;
//! impl TundraPreset for MyPreset {
//!		const DIGEST_SIZE: usize = 128;
//!		const LANE_SIZE: usize = 128;
//!		const COMPRESSION_ROUNDS: usize = 64;
//!		const DESCENT_COMPRESSION_ROUNDS: usize = 16;
//!		const INNER_PERMUTATION_ROUNDS: usize = 24;
//!		const OUTER_PERMUTATION_ROUNDS: usize = 16;
//!		const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize = 2;
//! } // Or make your own!
//!
//! let hash = Nessak::produce_hash::<MyPreset>(&[]); // And use it like any other preset
//! }
//! ```
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
