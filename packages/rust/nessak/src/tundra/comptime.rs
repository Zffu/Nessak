//! The constant implementation of Tundra.
//! Similar to the normal implementation but uses [`TundraPreset`] to store parameters rather than at runtime.

use alloc::vec;
use alloc::vec::Vec;

use crate::{
    preset::TundraPreset,
    tundra::{TundraConst, TundraImplementation},
    utils::math::lcm,
};

impl<I: TundraImplementation, P: TundraPreset> TundraConst<P> for I {
    fn sanitize_input(input: &[u8]) -> Vec<u8> {
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
        expanded_input.extend_from_slice(&P::DIGEST_SIZE.to_be_bytes());

        expanded_input
    }

    fn expand_input_buffer(input: &[u8]) -> Vec<u32> {
        let mut internal_state_size: usize = lcm(I::PERMUTATION_MUL_SIZE * 4, P::LANE_SIZE / 8);

        internal_state_size =
            ((input.len() * P::INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER + internal_state_size - 1)
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
            buffer.push(I::expand_generate(n, &buffer, input_word_count));
        }

        buffer
    }

    fn do_buffer_permutations(buffer: &mut [u32]) {
        let block_count = buffer.len() / I::PERMUTATION_MUL_SIZE;

        // Preallocate the capacity
        let mut harmonizer: Vec<u32> = vec![0; I::PERMUTATION_MUL_SIZE];

        for _ in 0..P::OUTER_PERMUTATION_ROUNDS {
            harmonizer.fill(0);

            for i in 0..block_count * I::PERMUTATION_MUL_SIZE {
                let y = i % I::PERMUTATION_MUL_SIZE;

                harmonizer[y] ^= buffer[i];
            }

            for x in 0..block_count {
                I::permutation_inner(
                    &mut buffer[x * I::PERMUTATION_MUL_SIZE..(x + 1) * I::PERMUTATION_MUL_SIZE],
                    &harmonizer,
                    P::INNER_PERMUTATION_ROUNDS,
                );
            }
        }
    }

    fn compress_buffer_into_internal_state(buffer: &mut [u32]) -> Vec<u32> {
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

            for r in 0..P::COMPRESSION_ROUNDS {
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

    fn descent_generation_round(lane_a: &mut [u32], lane_b: &[u32]) {
        let lane_expansion_count = lane_a.len() / (256 / 32);

        for r in 0..P::DESCENT_COMPRESSION_ROUNDS {
            for k in 0..lane_expansion_count {
                let p = k.saturating_sub(1);

                I::descent_compression_inner(lane_a, lane_b, k, p, r);
            }
        }
    }

    fn descend_internal_state(mut compression_set: Vec<u32>) -> Vec<u32> {
        let mut working_set = Vec::with_capacity(compression_set.len() / 2);

        let lane_size_in_words = P::LANE_SIZE / 32;

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

                <Self as Tundra<P>>::descent_generation_round(output, lane_b);
            }

            core::mem::swap(&mut compression_set, &mut working_set);
        }

        working_set.clear();
        working_set.extend_from_slice(&compression_set[0..lane_size_in_words]);

        working_set
    }

    fn get_digest(internal_state: Vec<u32>) -> Vec<u8> {
        let lane = <Self as Tundra<P>>::descend_internal_state(internal_state);

        let word_count_digest = ((P::DIGEST_SIZE + P::DIGEST_SIZE % 32) / 32) as usize;
        let mut digest = Vec::with_capacity(word_count_digest * 4);

        let mut ind = 0;
        while digest.len() < P::DIGEST_SIZE / 8 && ind < lane.len() {
            digest.extend_from_slice(&lane[ind].to_le_bytes());

            ind += 1;
        }

        digest[0..P::DIGEST_SIZE as usize / 8].to_vec()
    }

    fn produce_hash(input: &[u8]) -> Vec<u8> {
        const {
            assert!(P::DIGEST_SIZE != 0);
            assert!(P::LANE_SIZE != 0);
            assert!(P::COMPRESSION_ROUNDS != 0);
            assert!(P::DESCENT_COMPRESSION_ROUNDS != 0);
            assert!(P::INNER_PERMUTATION_ROUNDS != 0);
            assert!(P::OUTER_PERMUTATION_ROUNDS != 0);
            assert!(P::INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER != 0);

            assert!(P::LANE_SIZE >= P::DIGEST_SIZE as usize);
            assert!(P::LANE_SIZE % 256 == 0);
        }

        let input = <Self as Tundra<P>>::sanitize_input(input);

        let mut buffer = <Self as Tundra<P>>::expand_input_buffer(&input);

        <Self as Tundra<P>>::do_buffer_permutations(&mut buffer);

        let internal_state = <Self as Tundra<P>>::compress_buffer_into_internal_state(&mut buffer);

        return <Self as Tundra<P>>::get_digest(internal_state);
    }
}
