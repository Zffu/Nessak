//! The standalone implementation of the Nessak hash function

#[cfg(any(not(feature = "const"), doc))]
use alloc::vec::Vec;

#[cfg(any(not(feature = "const"), doc))]
use crate::{
    impls::{compress_inner, descent_compression_inner, expand_generate, permutation_inner},
    utils::math::lcm,
};

#[cfg(feature = "const")]
/// The const variant of the standalone implementation.
pub mod comptime;

/// The Nessak hash function.
/// ```
/// use nessak::standalone::Nessak;
///
/// let digest = Nessak::produce_hash(&[], 256, 256, 64, 8, 24, 4, 1);
///
/// assert_eq!(digest, [
/// 	0xC4, 0x6D, 0x62, 0x25, 0xFC, 0xD0, 0x91, 0x88, 0x61, 0xA1, 0x95, 0x89, 0xDE, 0x11,
/// 	0xC1, 0x64, 0x6E, 0x81, 0x24, 0xAF, 0x6C, 0xF3, 0xF2, 0x56, 0x4B, 0xBA, 0xA5, 0x1B,
/// 	0x94, 0x31, 0xCC, 0x74
/// ]);
/// ```
pub struct Nessak;

#[cfg(any(not(feature = "const"), doc))]
impl Nessak {
    fn sanitize_input(input: &[u8], digest_size: u64) -> Vec<u8> {
        let original_len = input.len();
        let mut expanded_input = input.to_vec();

        let mut padded_len = 73.max(original_len + 17);

        if padded_len % 4 != 0 {
            padded_len += 4 - (padded_len % 4);
        } // If padded len isn't divisible by 4, we make it

        expanded_input.push(0x00); // Delimiter

        while expanded_input.len() < padded_len - 16 {
            expanded_input.push(0x00); // Padding
        }

        expanded_input.extend_from_slice(&(original_len as u64).to_be_bytes());
        expanded_input.extend_from_slice(&digest_size.to_be_bytes());

        expanded_input
    }

    fn expand_input_buffer(
        input: &[u8],
        lane_size: usize,
        internal_state_minimum_length_multiplier: usize,
    ) -> Vec<u32> {
        let mut internal_state_size: usize = lcm(200, lane_size / 8);

        internal_state_size =
            ((input.len() * internal_state_minimum_length_multiplier + internal_state_size - 1)
                / internal_state_size)
                * internal_state_size;

        let input_word_count = input.len() / 4;

        let mut buffer = Vec::with_capacity(internal_state_size / 2);
        buffer.extend(
            input
                .chunks_exact(4)
                .map(|word| u32::from_be_bytes([word[0], word[1], word[2], word[3]])),
        );

        for n in input_word_count..(internal_state_size / 2) {
            buffer.push(expand_generate(n, &buffer, input_word_count));
        }

        buffer
    }

    fn do_buffer_permutations(
        buffer: &mut [u32],
        inner_permutation_rounds: usize,
        outer_permutation_rounds: usize,
    ) {
        let block_count = buffer.len() / 50;

        // Preallocate the capacity
        let mut harmonizer: [u32; 50] = [0; 50];

        for _ in 0..outer_permutation_rounds {
            harmonizer.fill(0);

            for i in 0..block_count * 50 {
                let y = i % 50;

                harmonizer[y] ^= buffer[i];
            }

            for x in 0..block_count {
                permutation_inner(
                    &mut buffer[x * 50..(x + 1) * 50],
                    &harmonizer,
                    inner_permutation_rounds,
                );
            }
        }
    }

    fn compress_buffer_into_internal_state(
        buffer: &mut [u32],
        compression_rounds: usize,
    ) -> Vec<u32> {
        let mut internal_state = Vec::with_capacity(buffer.len() / 2);

        let amount_of_parts = buffer.len() / 8;

        let mut part_a_index;
        let mut part_b_index;

        let mut k = 0;

        while k < amount_of_parts {
            part_a_index = k;

            if k == amount_of_parts - 1 {
                part_b_index = 0;
                k += 1;
            } else {
                part_b_index = k + 1;
                k += 2;
            }

            let part_a_offset = 8 * part_a_index;
            let part_b_offset = 8 * part_b_index;

            for r in 0..compression_rounds {
                let (part_a, part_b) = buffer.split_at_mut(part_a_offset + 8);
                let part_a = &mut part_a[part_a_offset..part_a_offset + 8];
                let part_b =
                    &part_b[part_b_offset - (part_a_offset + 8)..part_b_offset - part_a_offset];

                compress_inner(part_a, part_b, r);
            }

            internal_state.extend_from_slice(&buffer[part_a_offset..part_a_offset + 8]);
        }

        internal_state
    }

    fn descent_generation_round(
        lane_a: &mut [u32],
        lane_b: &[u32],
        descent_compression_rounds: usize,
    ) {
        let lane_expansion_count = lane_a.len() / (256 / 32);

        for r in 0..descent_compression_rounds {
            for k in 0..lane_expansion_count {
                let p = k.saturating_sub(1);

                descent_compression_inner(lane_a, lane_b, k, p, r);
            }
        }
    }

    fn descend_internal_state(
        mut compression_set: Vec<u32>,
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u32> {
        let mut working_set = Vec::with_capacity(compression_set.len() / 2);

        while compression_set.len() > lane_size_in_words {
            working_set.clear();

            for n in 0..compression_set.len() / (2 * lane_size_in_words) {
                let lane_a_start = n * 2 * lane_size_in_words;
                let lane_b_start = lane_a_start + lane_size_in_words;

                let lane_a = &compression_set[lane_a_start..lane_a_start + lane_size_in_words];
                let lane_b = &compression_set[lane_b_start..lane_b_start + lane_size_in_words];

                let output_start = working_set.len();
                working_set.resize(output_start + lane_size_in_words, 0);

                let output = &mut working_set[output_start..];

                output.copy_from_slice(&lane_a);

                Self::descent_generation_round(output, lane_b, descent_compression_rounds);
            }

            core::mem::swap(&mut compression_set, &mut working_set);
        }

        working_set.clear();
        working_set.extend_from_slice(&compression_set[0..lane_size_in_words]);

        working_set
    }

    fn get_digest(
        internal_state: Vec<u32>,
        digest_size: u64,
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u8> {
        let lane = Self::descend_internal_state(
            internal_state,
            lane_size_in_words,
            descent_compression_rounds,
        );

        let word_count_digest = ((digest_size + digest_size % 32) / 32) as usize;
        let mut digest = Vec::with_capacity(word_count_digest * 4);

        let mut ind = 0;
        while digest.len() < digest_size as usize / 8 && ind < lane.len() {
            digest.extend_from_slice(&lane[ind].to_le_bytes());

            ind += 1;
        }

        digest[0..digest_size as usize / 8].to_vec()
    }

    /// Produces a hash for the given input and given parameters using the standard Nessak algorithm.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(not(feature = "const"))] {
    ///	use nessak::standalone::Nessak;
    ///
    /// let digest = Nessak::produce_hash(&[], 128, 256, 64, 8, 24, 4, 1);
    ///
    /// # }
    /// ```
    ///
    pub fn produce_hash(
        input: &[u8],
        digest_size: u64,
        lane_size: usize,
        compression_rounds: usize,
        descent_compression_rounds: usize,
        inner_permutation_rounds: usize,
        outer_permutation_rounds: usize,
        internal_state_minimum_length_multiplier: usize,
    ) -> Vec<u8> {
        #[cfg(feature = "safety-checks")]
        {
            assert!(digest_size != 0);
            assert!(lane_size != 0);
            assert!(compression_rounds != 0);
            assert!(descent_compression_rounds != 0);
            assert!(inner_permutation_rounds != 0);
            assert!(outer_permutation_rounds != 0);
            assert!(internal_state_minimum_length_multiplier != 0);

            assert!(lane_size >= digest_size as usize);
            assert_eq!(lane_size % 256, 0);
        }

        let lane_size_in_words = lane_size / 32;

        let input = Self::sanitize_input(input, digest_size);

        let mut buffer =
            Self::expand_input_buffer(&input, lane_size, internal_state_minimum_length_multiplier);

        Self::do_buffer_permutations(
            &mut buffer,
            inner_permutation_rounds,
            outer_permutation_rounds,
        );

        let internal_state =
            Self::compress_buffer_into_internal_state(&mut buffer, compression_rounds);

        return Self::get_digest(
            internal_state,
            digest_size,
            lane_size_in_words,
            descent_compression_rounds,
        );
    }
}

macro_rules! make_helper_normal {
    ($($name: ident => ($lane: literal, $digest: literal)),* $(,)?) => {
		#[cfg(any(not(feature = "const"), doc))]
		impl Nessak {
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
		#[cfg(any(not(feature = "const"), doc))]
		impl Nessak {
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
