//!
//! The Nessak implementation of the Tundra structure.
//!

#[cfg(feature = "const")]
use alloc::vec::Vec;
#[cfg(not(feature = "const"))]
use alloc::vec::Vec;

#[cfg(not(feature = "const"))]
use crate::tundra::TundraRuntime;
#[cfg(feature = "const")]
use crate::{preset::TundraPreset, tundra::TundraConst};

use crate::{impls, tundra::TundraImplementation};

/// The implementation of the *Nessak* hash function using the *Tundra* structure implementation.
pub struct NessakTundraImplementation;

impl TundraImplementation for NessakTundraImplementation {
    const MINIMUM_EXPANSION_LEN: usize = 56;
    const PART_SIZE: usize = 8;
    const PERMUTATION_MUL_SIZE: usize = 50;

    #[inline(always)]
    fn expand_generate(n: usize, o: &[u32], w: usize) -> u32 {
        impls::expand_generate(n, o, w)
    }

    #[inline(always)]
    fn permutation_inner(state: &mut [u32], harmonizer: &[u32], inner_permutation_rounds: usize) {
        impls::permutation_inner(state, harmonizer, inner_permutation_rounds);
    }

    #[inline(always)]
    fn compress_inner(part_a: &mut [u32], part_b: &[u32], round: usize) {
        impls::compress_inner(part_a, part_b, round);
    }

    #[inline(always)]
    fn descent_compression_inner(lane_a: &mut [u32], lane_b: &[u32], k: usize, p: usize, r: usize) {
        impls::descent_compression_inner(lane_a, lane_b, k, p, r);
    }
}

macro_rules! make_helper_normal {
    ($($name: ident => ($lane: literal, $digest: literal)),* $(,)?) => {
		#[cfg(not(feature = "const"))]
		impl NessakTundraImplementation {
			$(
				#[doc = concat!("Helper for normal Nessak standard ", stringify!($name))]
				pub fn $name(input: &[u8]) -> Vec<u8> {
					Self::produce_hash(input, $digest, $lane, 64, 8, 24, 4, 1)
				}
			)*
		}

	};
}

macro_rules! make_helper_extended {
    ($($name: ident => ($lane: literal, $digest: literal)),* $(,)?) => {
		#[cfg(not(feature = "const"))]
		impl NessakTundraImplementation {
			$(
				#[doc = concat!("Helper for extended Nessak standard ", stringify!($name))]
				pub fn $name(input: &[u8]) -> Vec<u8> {
					Self::produce_hash(input, $digest, $lane, 64, 16, 24, 16, 2)
				}
			)*
		}

	};
}

make_helper_normal!(
    k2048_2048 => (2048, 2048),
    k1024_1024 => (1024, 1024),
    k512_512 => (512, 512),
    k256_256 => (256, 256),
    k256_128 => (256, 128),
    k256_64 => (256, 64),
    k256_32 => (256, 32),
    k256_16 => (256, 16)
);

make_helper_extended!(
    k4096_2048 => (4096, 2048),
    k2048_1024 => (2048, 1024),
    k1024_512 => (1024, 512),
    k512_256 => (512, 256),
    k512_128 => (512, 128),
    k512_64 => (512, 64),
    k512_32 => (512, 32),
    k512_16 => (512, 16)
);

#[cfg(feature = "const")]
impl NessakTundraImplementation {
    /// Small helper to use Nessak presets in a cleaner way.
    #[inline(always)]
    pub fn produce_hash<P: TundraPreset>(input: &[u8]) -> Vec<u8> {
        <NessakTundraImplementation as TundraConst<P>>::produce_hash(input)
    }
}
