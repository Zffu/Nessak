pub trait TundraPreset {
    const DIGEST_SIZE: usize;
    const LANE_SIZE: usize;
    const COMPRESSION_ROUNDS: usize;
    const DESCENT_COMPRESSION_ROUNDS: usize;
    const INNER_PERMUTATION_ROUNDS: usize;
    const OUTER_PERMUTATION_ROUNDS: usize;
    const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize;
}

macro_rules! make_preset_normal {
    ($lane_size: literal; $($struct_name:ident => $digest: literal),* $(,)?) => {
        $(
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
