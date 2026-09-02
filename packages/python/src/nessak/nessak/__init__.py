"""
The standalone implementation of Nessak
"""
import math

import numpy as np
from numpy.typing import NDArray

from ..impls import (
    compression_inner,
    descent_compression_inner,
    expand_generate,
    permutation_inner,
)


class Nessak: 
    def sanitize_input(self, input: list[int], digest_size: int) -> list[int]:
        """Sanitizes the input. Allowing it to be correctly passed to the hash function regardless of it's original size.

        Args:
            input (list[int]): the input as list[int]

        Returns:
            list[int]: the padded input as list[int].
        """
        input = input.copy()  # We copy since we shouldn't consume the input

        original_len = len(input)
        padded_len = max(73, original_len + 17)

        if padded_len % 4 != 0:
            padded_len += 4 - (
                padded_len % 4
            )  # If the padded length isn't divisible by 4, we make it

        input.append(0x00)  # Delimiter

        while len(input) < padded_len - 16:
            input.append(0x00)  # Append padding

        input += original_len.to_bytes(8, byteorder="big")  # Original size
        input += digest_size.to_bytes(8, byteorder="big")  # Digest size.

        return input

    def expand_input_buffer(
        self,
        input: list[int],
        lane_size: int,
        internal_state_minimum_length_multiplier: int,
    ) -> NDArray[np.uint32]:
        """Expands the input into an arbitrarily sized buffer

        Args:
                input (list[int]): the input as list[int]
                lane_size (int): the size of an individual descent lane (in bits)
                internal_state_minimum_length_multiplier(int): The multiplier of the minimum length of the internal state (based on the input length).

        Returns:
                list[int]: the buffer as words (32-bit integers)
        """

        internal_state_size = math.lcm(
            200, lane_size // 8
        )

        internal_state_size = (
            (
                len(input) * internal_state_minimum_length_multiplier
                + internal_state_size
                - 1
            )
            // internal_state_size
        ) * internal_state_size

        total_words: int = internal_state_size // 2

        input_words = len(input) // 4

        buffer = np.empty(total_words, dtype=np.uint32)

        buffer[:input_words] = np.frombuffer(bytes(input), dtype=">u4").astype(np.uint32)

        for n in range(input_words, total_words):
            buffer[n] = expand_generate(n, buffer, input_words)

        return buffer

    def do_buffer_permutations(self, buffer: NDArray[np.uint32], inner_permutation_rounds: int, outer_permutation_rounds: int):
        """Perform a given amount of permutation rounds on the provided buffer.

        Args:
            buffer (list[int]): the buffer to perform permutations on. The buffer will be updated.
            inner_permutation_rounds (int): the amount of inner rounds of permutations.
            outer_permutation_rounds (int): the amount of outer rounds of permutations.
        """

        blocks = buffer.reshape(-1, 50)

        for _ in range(outer_permutation_rounds):
            harmonizer = np.bitwise_xor.reduce(blocks, axis=0)

            for state in blocks:
                permutation_inner(state, harmonizer, inner_permutation_rounds)


    def compress_buffer_into_internal_state(self, buffer: NDArray[np.uint32], compression_rounds: int) -> NDArray[np.uint32]:
        """Compresses the buffer into the internal state. A buffer is twice as big as an internal state

        Args:
            buffer (list[int]): The buffer
            compression_rounds (int): the amount of compression rounds

        Returns:
            list[int]: the internal state (as an array of words)
        """
        internal_state: NDArray[np.uint32] = np.empty(len(buffer) // 2, dtype=np.uint32)

        parts = buffer.reshape(-1, 8)

        amount_of_parts = len(buffer) // 8

        part_a_index = 0
        part_b_index = 0

        k = 0
        i = 0

        while k < amount_of_parts:
            part_a_index = k

            if k == amount_of_parts - 1:
                part_b_index = 0
                k += 1
            else:
                part_b_index = k + 1
                k += 2

            part_a_offset = 8 * part_a_index

            for r in range(compression_rounds):
                compression_inner(parts[part_a_index], parts[part_b_index], r)

            internal_state[i * 8:(i + 1)*8] = buffer[part_a_offset:part_a_offset + 8] 

            i += 1

        return internal_state

    def descent_generation_round(self, lane_a: NDArray[np.uint32], lane_b: NDArray[np.uint32], descent_compression_rounds: int) -> NDArray[np.uint32]:
        """Performs a descent generation round of lane A and B.

        Args:
            lane_a (list[int]): The lane A.
            lane_b (list[int]): The lane B.
            descent_compression_rounds (int): The amount of compression rounds inside of the descent generation

        Returns:
            list[int]: The lane result of the compression between the two lanes
        """

        lane_expansion_count = lane_a.size // (256 // 32)

        for r in range(descent_compression_rounds):
            for k in range(lane_expansion_count):
                p = max(0, k - 1)

                ok = 8 * k

                result = descent_compression_inner(lane_a, lane_b, k, p, r)

                for i in range(8):
                    lane_a[ok + i] = result[i]

        return lane_a.copy()

    def descend_internal_state(
        self,
        compression_set: NDArray[np.uint32],
        lane_size_in_words: int,
        descent_compression_rounds: int,
    ) -> NDArray[np.uint32]:

        working_set = np.empty_like(compression_set)

        compression_set_len = len(compression_set)

        while compression_set_len > lane_size_in_words:
            lane_count = compression_set_len // lane_size_in_words

            for n in range(lane_count // 2):
                start_a = n * 2 * lane_size_in_words
                start_b = start_a + lane_size_in_words
                start_out = n * lane_size_in_words

                lane_a = compression_set[
                    start_a:start_a + lane_size_in_words
                ]
                lane_b = compression_set[
                    start_b:start_b + lane_size_in_words
                ]

                working_set[
                    start_out:start_out + lane_size_in_words
                ] = self.descent_generation_round(
                    lane_a,
                    lane_b,
                    descent_compression_rounds,
                )

            compression_set_len //= 2
            compression_set, working_set = working_set, compression_set

        return compression_set[:lane_size_in_words]

    def get_digest(self, internal_state: NDArray[np.uint32], digest_size: int, lane_size_in_words: int, descent_compression_rounds: int) -> list[int]:
        """Performs descent and gathers the digest from the final lane.

        Args:
            internal_state (list[int]): The internal state
            digest_size (int): The size of the digest (in bits)
            lane_size_in_words (int): The size of a descent lane (in bits)
            descent_compression_rounds (int): The amount of compression rounds per descent round

        Returns:
            list[int]: The digest
        """
        lane = self.descend_internal_state(internal_state, lane_size_in_words, descent_compression_rounds)

        digest = lane.astype("<u4").tobytes()

        return list(digest[:digest_size // 8])

    def produce_hash(self, input: list[int], digest_size: int = 256, lane_size: int = 256, compression_rounds: int = 64, descent_compression_rounds: int = 8, inner_permutation_rounds: int = 24, outer_permutation_rounds: int = 4, internal_state_minimum_length_multiplier: int = 1) -> list[int]:
        """Uses the Tundra implementation to generate the hash of the given input.

        Args:
            input (list[int]): The input fed to generate the hash. The fed input will be modified
            digest_size (int, optional): The size of the digest (in bits). Defaults to 256
            lane_size (int, optional): The size of a single descent lane. Defaults to 256.
            compression_rounds (int, optional): The amount of rounds of the compression. Defaults to 64.
            descent_compression_rounds (int, optional): The amount of descent compression rounds. Defaults to 8.
            inner_permutation_rounds (int, optional): The amount of inner permutation rounds. Defaults to 24.
            outer_permutation_rounds (int, optional): The amount of outer permutation rounds. Defaults to 4.
            internal_state_minimum_length_multiplier (int, optional): The multiplier for the minimum length of the internal state. Defaults to 1.

        Returns:
            list[int]: The output digest in list[int]
        """

        assert digest_size > 0
        assert lane_size > 0
        assert compression_rounds > 0
        assert descent_compression_rounds > 0
        assert inner_permutation_rounds > 0
        assert outer_permutation_rounds > 0
        assert internal_state_minimum_length_multiplier > 0

        assert lane_size >= digest_size
        assert lane_size % 256 == 0

        lane_size_in_words = lane_size // 32

        input = self.sanitize_input(input, digest_size)

        buffer = self.expand_input_buffer(input, lane_size, internal_state_minimum_length_multiplier)
        self.do_buffer_permutations(buffer, inner_permutation_rounds, outer_permutation_rounds)

        internal_state = self.compress_buffer_into_internal_state(buffer, compression_rounds)
 
        return self.get_digest(internal_state,  digest_size, lane_size_in_words, descent_compression_rounds)

    def k2048_2048(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=2048, lane_size=2048)

    def k1024_1024(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=1024, lane_size=1024)

    def k512_512(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=512, lane_size=512)

    def k256_256(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=256)

    def k256_128(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=128)

    def k256_64(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=64)

    def k256_32(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=32)

    def k256_16(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=16)

    def k4096_2048(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=2048, lane_size=4096, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k2048_1024(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=1024, lane_size=2048, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k1024_512(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=512, lane_size=1024, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k512_256(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=256, lane_size=512, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k512_128(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=128, lane_size=512, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k512_64(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=64, lane_size=512, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k512_32(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=32, lane_size=512, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)

    def k512_16(self, input: list[int]) -> list[int]:
        return self.produce_hash(input, digest_size=16, lane_size=512, descent_compression_rounds=16, outer_permutation_rounds=16, internal_state_minimum_length_multiplier=2)
