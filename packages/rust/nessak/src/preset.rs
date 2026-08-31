//! The configuration presets for the const versions of the implementations.
//! There is one preset per "recognized" Nessak standard with the ability of creating your own.

/// Represents a parameter preset inside of a constant implementation inside of this crate.
/// Normally, these values would be provided at runtime but these allow to pass them at compile time on a const implementation.
pub trait TundraPreset {
    /// The size of the digest (in bits).
    const DIGEST_SIZE: usize;

    /// The size of each descent lane (in bits).
    const LANE_SIZE: usize;

    /// The amount of rounds of compression.
    const COMPRESSION_ROUNDS: usize;

    /// The amount of rounds of descent compression.
    const DESCENT_COMPRESSION_ROUNDS: usize;

    /// The amount of rounds of inner permutations.
    const INNER_PERMUTATION_ROUNDS: usize;

    /// The amount of rounds of outer permutation.
    const OUTER_PERMUTATION_ROUNDS: usize;

    /// The multiplier for the minimum size of the internal size based on input size.
    const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize;
}

macro_rules! make_preset_normal {
    ($lane_size: literal; $($struct_name:ident => $digest: literal),* $(,)?) => {
        $(
			#[doc = concat!("Parameter preset for Nessak normal preset ", stringify!($struct_name))]
			pub struct $struct_name;

			impl TundraPreset for $struct_name {
				const DIGEST_SIZE: usize = $digest;
				const LANE_SIZE: usize = $lane_size;
				const COMPRESSION_ROUNDS: usize = 64;
				const DESCENT_COMPRESSION_ROUNDS: usize = 8;
				const INNER_PERMUTATION_ROUNDS: usize = 24;
				const OUTER_PERMUTATION_ROUNDS: usize = 4;
				const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize = 1;
			}
		)*
    };
}

macro_rules! make_preset_extended {
    ($lane_size: literal; $($struct_name:ident => $digest: literal),* $(,)?) => {
        $(
			#[doc = concat!("Parameter preset for Nessak extended preset ", stringify!($struct_name))]
			pub struct $struct_name;

			impl TundraPreset for $struct_name {
				const DIGEST_SIZE: usize = $digest;
				const LANE_SIZE: usize = $lane_size;
				const COMPRESSION_ROUNDS: usize = 64;
				const DESCENT_COMPRESSION_ROUNDS: usize = 16;
				const INNER_PERMUTATION_ROUNDS: usize = 24;
				const OUTER_PERMUTATION_ROUNDS: usize = 16;
				const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize = 2;
			}
		)*
    };
}

make_preset_normal!(2048; NessakK2048_2048 => 2048);
make_preset_normal!(1024; NessakK1024_1024 => 1024);
make_preset_normal!(512; NessakK512_512 => 512);
make_preset_normal!(
    256;
    NessakK256_256 => 256,
    NessakK256_128 => 128,
    NessakK256_64 => 64,
    NessakK256_32 => 32,
    NessakK256_16 => 16
);

// Extended Presets

make_preset_extended!(4096; NessakK4096_2048 => 2048);
make_preset_extended!(2048; NessakK2048_1024 => 1024);
make_preset_extended!(1024; NessakK1024_512 => 512);
make_preset_extended!(
    512;
    NessakK512_256 => 256,
    NessakK512_128 => 128,
    NessakK512_64 => 64,
    NessakK512_32 => 32,
    NessakK512_16 => 16
);
