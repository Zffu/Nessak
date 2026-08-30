use std::marker::PhantomData;

use crate::utils::lcm;

pub mod nessak;

pub trait TundraImplementation {
    fn expand_generate(n: usize, o: &[u32], w: usize) -> u32;

    fn permutation_inner(state: &mut [u32], harmonizer: &[u32], inner_permutation_rounds: usize);

    fn compress_inner(part_a: &mut [u32], part_b: &[u32], round: usize);

    fn descent_compression_inner(lane_a: &mut [u32], lane_b: &[u32], k: usize, p: usize, r: usize);
}

pub struct Tundra<I: TundraImplementation> {
    minimum_expansion_len: usize,
    permutation_mul_size: usize,
    part_size: usize,

    __marker: PhantomData<I>,
}

impl<I: TundraImplementation> Tundra<I> {
    pub fn new(
        minimum_expansion_len: usize,
        permutation_mul_size: usize,
        part_size: usize,
    ) -> Self {
        Self {
            minimum_expansion_len,
            permutation_mul_size,
            part_size,
            __marker: PhantomData,
        }
    }

    pub(crate) fn sanitize_input(&self, input: &[u8], digest_size: u64) -> Vec<u8> {
        let original_len = input.len();
        let mut expanded_input = input.to_vec();

        let mut padded_len = (self.minimum_expansion_len + 17).max(original_len + 17);

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

    pub(crate) fn expand_input_buffer(
        &self,
        input: &[u8],
        lane_size: usize,
        internal_state_minimum_length_multiplier: usize,
    ) -> Vec<u32> {
        let mut buffer: Vec<u32> = input
            .chunks_exact(4)
            .map(|word| u32::from_be_bytes(word.try_into().unwrap()))
            .collect();

        let mut internal_state_size: usize = lcm(lcm(self.permutation_mul_size, 4), lane_size / 8);

        internal_state_size =
            ((input.len() * internal_state_minimum_length_multiplier + internal_state_size - 1)
                / internal_state_size)
                * internal_state_size;

        let input_word_count = input.len() / 4;

        for n in input_word_count..(internal_state_size / 2) {
            buffer.push(I::expand_generate(n, &buffer, input_word_count));
        }

        buffer
    }

    pub(crate) fn do_buffer_permutations(
        &self,
        buffer: &mut [u32],
        inner_permutation_rounds: usize,
        outer_permutation_rounds: usize,
    ) {
        let block_count = buffer.len() / self.permutation_mul_size;

        // Preallocate the capacity
        let mut harmonizer: Vec<u32> = vec![0; self.permutation_mul_size];

        for _ in 0..outer_permutation_rounds {
            harmonizer.fill(0);

            for i in 0..block_count * self.permutation_mul_size {
                let y = i % self.permutation_mul_size;

                harmonizer[y] ^= buffer[i];
            }

            for x in 0..block_count {
                I::permutation_inner(
                    &mut buffer[x * self.permutation_mul_size..(x + 1) * self.permutation_mul_size],
                    &harmonizer,
                    inner_permutation_rounds,
                );
            }
        }
    }

    pub(crate) fn compress_buffer_into_internal_state(
        &self,
        buffer: &mut [u32],
        compression_rounds: usize,
    ) -> Vec<u32> {
        let mut internal_state = vec![];

        let amount_of_parts = buffer.len() / self.part_size;

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

            let part_a_offset = self.part_size * part_a_index;
            let part_b_offset = self.part_size * part_b_index;

            for r in 0..compression_rounds {
                let (part_a, part_b) = buffer.split_at_mut(part_a_offset + self.part_size);
                let part_a = &mut part_a[part_a_offset..part_a_offset + self.part_size];
                let part_b = &part_b[part_b_offset - (part_a_offset + self.part_size)
                    ..part_b_offset - part_a_offset];

                I::compress_inner(part_a, part_b, r);
            }

            internal_state
                .extend_from_slice(&buffer[part_a_offset..part_a_offset + self.part_size]);
        }

        internal_state
    }

    pub(crate) fn descent_generation_round(
        &self,
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

    pub(crate) fn descend_internal_state(
        &self,
        internal_state: &[u32],
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u32> {
        let mut compression_set = internal_state.to_vec();
        let mut working_set = Vec::new();

        let mut result_lane_set = vec![0_u32; lane_size_in_words];

        while compression_set.len() > lane_size_in_words {
            working_set.clear();

            for n in 0..compression_set.len() / (2 * lane_size_in_words) {
                let lane_a_start = n * 2 * lane_size_in_words;
                let lane_b_start = lane_a_start + lane_size_in_words;

                let lane_a = &compression_set[lane_a_start..lane_a_start + lane_size_in_words];
                let lane_b = &compression_set[lane_b_start..lane_b_start + lane_size_in_words];

                result_lane_set.copy_from_slice(&lane_a);

                self.descent_generation_round(
                    &mut result_lane_set,
                    lane_b,
                    descent_compression_rounds,
                );

                working_set.extend_from_slice(&result_lane_set);
            }

            std::mem::swap(&mut compression_set, &mut working_set);
        }

        working_set.clear();
        working_set.extend_from_slice(&compression_set[0..lane_size_in_words]);

        working_set
    }

    pub(crate) fn get_digest(
        &self,
        internal_state: &[u32],
        digest_size: u64,
        lane_size_in_words: usize,
        descent_compression_rounds: usize,
    ) -> Vec<u8> {
        let lane = self.descend_internal_state(
            internal_state,
            lane_size_in_words,
            descent_compression_rounds,
        );

        let mut digest = vec![];

        for num in &lane {
            digest.extend_from_slice(&num.to_le_bytes());
        }

        digest[0..digest_size as usize / 8].to_vec()
    }

    pub fn produce_hash(
        &self,
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

        let input = self.sanitize_input(input, digest_size);

        let mut buffer =
            self.expand_input_buffer(&input, lane_size, internal_state_minimum_length_multiplier);

        self.do_buffer_permutations(
            &mut buffer,
            inner_permutation_rounds,
            outer_permutation_rounds,
        );

        let internal_state =
            self.compress_buffer_into_internal_state(&mut buffer, compression_rounds);

        return self.get_digest(
            &internal_state,
            digest_size,
            lane_size_in_words,
            descent_compression_rounds,
        );
    }
}
