import math
from abc import ABC, abstractmethod


class Tundra(ABC):
    def __init__(self, minimum_expansion_len: int, permutation_mul_size: int, part_size: int) -> None:
        self.minimum_expansion_len = minimum_expansion_len
        self.permutation_mul_size = permutation_mul_size
        self.part_size = part_size

    def sanitize_input(self, input: list[int], digest_size: int) -> list[int]:
        """Sanitizes the input. Allowing it to be correctly passed to the hash function regardless of it's original size.

        Args:
            input (list[int]): the input as bytes

        Returns:
            list[int]: the padded input as bytes.
        """
        input = input.copy()  # We copy since we shouldn't consume the input

        original_len = len(input)
        padded_len = max(self.minimum_expansion_len + 17, original_len + 17)

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
    ) -> list[int]:
        """Expands the input into an arbitrarily sized buffer

        Args:
                input (list[int]): the input as bytes
                lane_size (int): the size of an individual descent lane (in bits)
                internal_state_minimum_length_multiplier(int): The multiplier of the minimum length of the internal state (based on the input length).

        Returns:
                list[int]: the buffer as words (32-bit integers)
        """
        buffer: list[int] = []
        input_words: list[int] = []

        internal_state_size = math.lcm(
            4 * self.permutation_mul_size, lane_size // 8
        )

        internal_state_size = (
            (
                len(input) * internal_state_minimum_length_multiplier
                + internal_state_size
                - 1
            )
            // internal_state_size
        ) * internal_state_size

        for i in range(0, len(input), 4):
            input_words.append(int.from_bytes(input[i : i + 4], byteorder="big"))

        buffer = input_words.copy()

        total_words: int = internal_state_size // 2

        for n in range(len(input_words), total_words):
            buffer.append(self.function_g(n, buffer, len(input_words)))

        return buffer

    def do_buffer_permutations(self, buffer: list[int], inner_permutation_rounds: int, outer_permutation_rounds: int):
        """Perform a given amount of permutation rounds on the provided buffer.

        Args:
            buffer (list[int]): the buffer to perform permutations on. The buffer will be updated.
            inner_permutation_rounds (int): the amount of inner rounds of permutations.
            outer_permutation_rounds (int): the amount of outer rounds of permutations.
        """

        block_count = len(buffer) // self.permutation_mul_size

        for _ in range(outer_permutation_rounds):
            state = [0] * self.permutation_mul_size
            harmonizer = [0] * self.permutation_mul_size

            for i in range(self.permutation_mul_size):
                harmonizer[i] = buffer[i]

            for x in range(1, block_count):
                for i in range(self.permutation_mul_size):
                    harmonizer[i] ^= buffer[self.permutation_mul_size * x + i]

            for x in range(block_count):
                for i in range(self.permutation_mul_size):
                    state[i] = buffer[x * self.permutation_mul_size + i]

                self.function_p(state, harmonizer, inner_permutation_rounds)

                for i in range(self.permutation_mul_size):
                    buffer[x * self.permutation_mul_size + i] = state[i]

    def compress_buffer_into_internal_state(self, buffer: list[int], compression_rounds: int) -> list[int]:
        """Compresses the buffer into the internal state. A buffer is twice as big as an internal state

        Args:
            buffer (list[int]): The buffer
            compression_rounds (int): the amount of compression rounds

        Returns:
            list[int]: the internal state (as an array of words)
        """
        internal_state: list[int] = []

        amount_of_parts = len(buffer) // self.part_size

        part_a_index = 0
        part_b_index = 0

        k = 0

        while k < amount_of_parts:
            part_a_index = k

            if k == amount_of_parts - 1:
                part_b_index = 0
                k += 1
            else:
                part_b_index = k + 1
                k += 2

            part_a_offset = self.part_size * part_a_index
            part_b_offset = self.part_size * part_b_index

            for r in range(compression_rounds):
                part_a = buffer[part_a_offset:part_a_offset + self.part_size]
                part_b = buffer[part_b_offset:part_b_offset + self.part_size]

                self.function_c(part_a, part_b, r)

                buffer[part_a_offset:part_a_offset + self.part_size] = part_a
                buffer[part_b_offset:part_b_offset + self.part_size] = part_b

            for i in range(self.part_size):
                internal_state.append(buffer[part_a_offset + i])

        return internal_state

    def descent_generation_round(self, lane_a: list[int], lane_b: list[int], descent_compression_rounds: int) -> list[int]:
        """Performs a descent generation round of lane A and B.

        Args:
            lane_a (list[int]): The lane A.
            lane_b (list[int]): The lane B.
            descent_compression_rounds (int): The amount of compression rounds inside of the descent generation

        Returns:
            list[int]: The lane result of the compression between the two lanes
        """

        lane_expansion_count = len(lane_a) // (256 // 32)

        for r in range(descent_compression_rounds):
            for k in range(lane_expansion_count):
                p = max(0, k - 1)

                ok = 8 * k

                result = self.function_d(lane_a, lane_b, k, p, r)

                for i in range(8):
                    lane_a[ok + i] = result[i]

        return lane_a.copy()

    def descend_internal_state(self, internal_state: list[int], lane_size_in_words: int, descent_compression_rounds: int) -> list[int]:
        """Performs the descent stage of the internal state

        Args:
            internal_state (list[int]): The internal state
            lane_size_in_words (int): The size of a descent lane in words (32-bit integers)
            descent_compression_rounds (int): The amount of compression rounds in each descent round.

        Returns:
            list[int]: The final lane.
        """
        working_set: list[list[int]] = []
        compression_set: list[list[int]] = []

        for i in range(0, len(internal_state), lane_size_in_words):
            compression_set.append(internal_state[i:i + lane_size_in_words])

        while len(compression_set) > 1:
            for n in range(len(compression_set) // 2):
                lane_a = compression_set[n * 2]
                lane_b = compression_set[n * 2 + 1]

                working_set.append(self.descent_generation_round(lane_a, lane_b, descent_compression_rounds))

            compression_set = working_set.copy()
            working_set.clear()

        return compression_set[0]

    def get_digest(self, internal_state: list[int], digest_size: int, lane_size_in_words: int, descent_compression_rounds: int) -> list[int]:
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

        digest: list[int] = []

        for k in lane:
            digest += k.to_bytes(4, 'little')

        return digest[0:digest_size // 8]

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
            list[int]: The output digest in bytes
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

    @abstractmethod
    def function_g(self, n: int, o: list[int], w: int) -> int:
        """The G function of the Tundra construction"""

    @abstractmethod
    def function_p(self, state: list[int], harmonizer: list[int], inner_permutation_rounds: int):
        """The P function of the Tundra construction"""

    @abstractmethod
    def function_c(self, part_a: list[int], part_b: list[int], round: int):
        """The C function of the Tundra construction"""
    
    @abstractmethod
    def function_d(self, lane_a: list[int], lane_b: list[int], k: int, p: int, r: int) -> list[int]:
        """The P function of the Tundra construction"""
