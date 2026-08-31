use alloc::vec;
use alloc::vec::Vec;

use crate::utils::math::lcm;

pub mod nessak;

pub trait TundraImplementation {
    const MINIMUM_EXPANSION_LEN: usize;
    const PART_SIZE: usize;
    const PERMUTATION_MUL_SIZE: usize;

    fn expand_generate(n: usize, o: &[u32], w: usize) -> u32;

    fn permutation_inner(state: &mut [u32], harmonizer: &[u32], inner_permutation_rounds: usize);

    fn compress_inner(part_a: &mut [u32], part_b: &[u32], round: usize);

    fn descent_compression_inner(lane_a: &mut [u32], lane_b: &[u32], k: usize, p: usize, r: usize);
}

pub trait Tundra {
    fn sanitize_input(input: &[u8], digest_size: u64) -> Vec<u8>;

    fn expand_input_buffer(
        input: &[u8],
        lane_size: usize,
        internal_state_minimum_length_multiplier: usize,
    ) -> Vec<u32>;

    fn do_buffer_permutations(
        buffer: &mut [u32],
        inner_permutation_rounds: usize,
        outer_permutation_rounds: usize,
    );

    fn compress_buffer_into_internal_state(
        buffer: &mut [u32],
        compression_rounds: usize,
    ) -> Vec<u32>;

    fn descent_generation_round(
        lane_a: &mut [u32],
        lane_b: &[u32],
        descent_compression_rounds: usize,
    );

    fn descend_internal_state(
        internal_state: &[u32],
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u32>;

    fn get_digest(
        internal_state: &[u32],
        digest_size: u64,
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u8>;

    fn produce_hash(
        input: &[u8],
        digest_size: u64,
        lane_size: usize,
        compression_rounds: usize,
        descent_compression_rounds: usize,
        inner_permutation_rounds: usize,
        outer_permutation_rounds: usize,
        internal_state_minimum_length_multiplier: usize,
    ) -> Vec<u8>;
}

impl<I: TundraImplementation> Tundra for I {
    fn sanitize_input(input: &[u8], digest_size: u64) -> Vec<u8> {
        let original_len = input.len();
        let mut expanded_input = input.to_vec();

        let mut padded_len = (I::MINIMUM_EXPANSION_LEN + 17).max(original_len + 17);

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
        let mut internal_state_size: usize = lcm(I::PERMUTATION_MUL_SIZE * 4, lane_size / 8);

        internal_state_size =
            ((input.len() * internal_state_minimum_length_multiplier + internal_state_size - 1)
                / internal_state_size)
                * internal_state_size;

        let input_word_count = input.len() / 4;

        let mut buffer = Vec::with_capacity(internal_state_size * 2);
        buffer.extend(
            input
                .chunks_exact(4)
                .map(|word| u32::from_be_bytes(word.try_into().unwrap())),
        );

        for n in input_word_count..(internal_state_size / 2) {
            buffer.push(I::expand_generate(n, &buffer, input_word_count));
        }

        buffer
    }

    fn do_buffer_permutations(
        buffer: &mut [u32],
        inner_permutation_rounds: usize,
        outer_permutation_rounds: usize,
    ) {
        let block_count = buffer.len() / I::PERMUTATION_MUL_SIZE;

        // Preallocate the capacity
        let mut harmonizer: Vec<u32> = vec![0; I::PERMUTATION_MUL_SIZE];

        for _ in 0..outer_permutation_rounds {
            harmonizer.fill(0);

            for i in 0..block_count * I::PERMUTATION_MUL_SIZE {
                let y = i % I::PERMUTATION_MUL_SIZE;

                harmonizer[y] ^= buffer[i];
            }

            for x in 0..block_count {
                I::permutation_inner(
                    &mut buffer[x * I::PERMUTATION_MUL_SIZE..(x + 1) * I::PERMUTATION_MUL_SIZE],
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

        let amount_of_parts = buffer.len() / I::PART_SIZE;

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

            let part_a_offset = I::PART_SIZE * part_a_index;
            let part_b_offset = I::PART_SIZE * part_b_index;

            for r in 0..compression_rounds {
                let (part_a, part_b) = buffer.split_at_mut(part_a_offset + I::PART_SIZE);
                let part_a = &mut part_a[part_a_offset..part_a_offset + I::PART_SIZE];
                let part_b = &part_b
                    [part_b_offset - (part_a_offset + I::PART_SIZE)..part_b_offset - part_a_offset];

                I::compress_inner(part_a, part_b, r);
            }

            internal_state.extend_from_slice(&buffer[part_a_offset..part_a_offset + I::PART_SIZE]);
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
                let p = 0_i64.max(k as i64 - 1) as usize;

                I::descent_compression_inner(lane_a, lane_b, k, p, r);
            }
        }
    }

    fn descend_internal_state(
        internal_state: &[u32],
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u32> {
        let mut compression_set = internal_state.to_vec();
        let mut working_set = Vec::with_capacity(internal_state.len() / 2);

        let mut result_lane_set = vec![0_u32; lane_size_in_words];

        while compression_set.len() > lane_size_in_words {
            working_set.clear();

            for n in 0..compression_set.len() / (2 * lane_size_in_words) {
                let lane_a_start = n * 2 * lane_size_in_words;
                let lane_b_start = lane_a_start + lane_size_in_words;

                let lane_a = &compression_set[lane_a_start..lane_a_start + lane_size_in_words];
                let lane_b = &compression_set[lane_b_start..lane_b_start + lane_size_in_words];

                result_lane_set.copy_from_slice(&lane_a);

                Self::descent_generation_round(
                    &mut result_lane_set,
                    lane_b,
                    descent_compression_rounds,
                );

                working_set.extend_from_slice(&result_lane_set);
            }

            core::mem::swap(&mut compression_set, &mut working_set);
        }

        working_set.clear();
        working_set.extend_from_slice(&compression_set[0..lane_size_in_words]);

        working_set
    }

    fn get_digest(
        internal_state: &[u32],
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

    #[inline(never)]
    fn produce_hash(
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
            &internal_state,
            digest_size,
            lane_size_in_words,
            descent_compression_rounds,
        );
    }
}
