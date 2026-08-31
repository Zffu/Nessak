from ..impls import (
    compression_inner,
    descent_compression_inner,
    expand_generate,
    permutation_inner,
)
from .tundra import Tundra


class Nessak(Tundra):
    def __init__(self):
        self.minimum_expansion_len = 56
        self.permutation_mul_size = 50
        self.part_size = 8

    def function_g(self, n: int, o: list[int], w: int) -> int:
        return expand_generate(n, o, w)

    def function_p(self, state: list[int], harmonizer: list[int], inner_permutation_rounds: int):
        return permutation_inner(state, harmonizer, inner_permutation_rounds)

    def function_c(self, part_a: list[int], part_b: list[int], round: int):
        return compression_inner(part_a, part_b, round)

    def function_d(self, lane_a: list[int], lane_b: list[int], k: int, p: int, r: int) -> list[int]:
        return descent_compression_inner(lane_a, lane_b, k, p, r)

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
