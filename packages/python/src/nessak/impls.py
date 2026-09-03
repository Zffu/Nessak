import numpy as np
from numpy.typing import NDArray

PERMUTATION_ROT = np.array(
    [
        0, 36, 3, 41, 18,
    	1, 44, 10, 45, 2,
        62, 6, 43, 15, 61,
        28, 55, 25, 21, 56,
        27, 20, 39, 8, 14
    ], dtype=np.uint32)

PERMUTATION_RC = [
	0x0000000000000001,
	0x0000000000008082,
	0x800000000000808A,
	0x8000000080008000,
	0x000000000000808B,
	0x0000000080000001,
	0x8000000080008081,
	0x8000000000008009,
	0x000000000000008A,
	0x0000000000000088,
	0x0000000080008009,
	0x000000008000000A,
	0x000000008000808B,
	0x800000000000008B,
	0x8000000000008089,
	0x8000000000008003,
	0x8000000000008002,
	0x8000000000000080,
	0x000000000000800A,
	0x800000008000000A,
	0x8000000080008081,
	0x8000000000008080,
	0x0000000080000001,
	0x8000000080008008
]

COMPRESSION_CONSTANTS = [
	0xCBBB9D5D, 0x629A292A, 0x9159015A,
	0x152FECD8, 0x67332667, 0x8EB44A87,
	0xDB0C2E0D, 0x47B5481D, 0xAE5F9156,
	0xCF6C85D3, 0x2F73477D, 0x6D1826CA,
	0x8B43D457, 0xE360B596, 0x1C456002,
	0x6F196331, 0xD94EBEB1, 0x0CC4A611,
	0x261DC1F2, 0x5815A7BE, 0x70B7ED67,
	0xA1513C69, 0x44F93635, 0x720DCDFD,
	0xB467369E, 0xCA320B75, 0x34E0D42E,
	0x49C7D9BD, 0x87ABB9F2, 0xC463A2FC,
	0xEC3FC3F3, 0x27277F6D, 0x610BEBF2,
	0x7420B49E, 0xD1FD8A33, 0xE4773594,
	0x092197F6, 0x1B530C95, 0x869D6342,
	0xEEE52E4F, 0x11076689, 0x21FBA37B,
	0x43AB9FB6, 0x75A9F91D, 0x86305019,
	0xD7CD8173, 0x07FE00FF, 0x379F513F,
	0x66B651A8, 0x764AB842, 0xA4B06BE1,
	0xC3578C15, 0xD2962A53, 0x1E039F40,
	0x857B7BEE, 0xA29BF2DE, 0xB11A32E8,
	0xCDF34E80, 0x31830426, 0x5B89092B,
	0xA0C06A13, 0xAE79842F, 0xC9CDA689,
	0xF281F239
]

DESCENT_COMPRESSION_CONSTANTS = [
	0x6A09E667,
	0xBB67AE85,
	0x3C6EF372,
	0xA54FF53A,
	0x510E527F,
	0x9B05688C,
	0x1F83D9AB,
	0x5BE0CD19,
]

MASK64 = np.uint64((1 << 64) - 1)
MASK32 = np.uint32((1 << 32) - 1)


def rotr(x: np.uint32, n: np.uint32) -> np.uint32:
	return ((x >> n) | (x << (32 - n))) & MASK32

def zeta0(x: np.uint32) -> np.uint32:
	return (rotr(x, np.uint32(2)) ^ rotr(x, np.uint32(7)) ^ rotr(x, np.uint32(17)) ^ (x >> np.uint32(3)) & MASK32) & MASK32

def zeta1(x: np.uint32) -> np.uint32:
	return (rotr(x, np.uint32(17)) ^ rotr(x, np.uint32(19)) ^ (x >> np.uint32(11)) & MASK32)

def alpha0(x: np.uint32, y: np.uint32, z: np.uint32) -> np.uint32:
	return ((x & rotr(y, np.uint32(3))) ^ ((~x) & z)) & MASK32

def beta(x: np.uint32, y: np.uint32, z: np.uint32) -> np.uint32:
	return ((x & y) ^ (z & x) ^ (y & z)) & MASK32

def rotl_64(x: np.uint32, n: np.uint32) -> np.uint32:
	return ((x << n) | (x >> (64 - n))) & MASK64

def expand_generate(n: int, o: NDArray[np.uint32], w: int) -> np.uint32:
	return np.add(o[n - w], np.add(zeta0(o[n - 14]), np.add(zeta1(o[n - 2]), o[n - 6], dtype=np.uint32), dtype=np.uint32), dtype=np.uint32)
	  
	#return (o[n - w] + zeta0(o[n - 14]) + zeta1(o[n - 2]) + o[n - 6]) & MASK32

def permutation_inner(state: NDArray[np.uint32], harmonizer: NDArray[np.uint32], inner_permutation_rounds: int):
    state_64 = np.empty(25, np.uint64)
    harmonizer_64 = np.empty(25, np.uint64)

    for i in range(25):
        state_64[i] = np.uint64(state[i * 2])
        state_64[i] |= np.uint64(state[i * 2 + 1]) << 32
        harmonizer_64[i] = np.uint64(harmonizer[i * 2])
        harmonizer_64[i] |= np.uint64(harmonizer[i * 2 + 1]) << 32
            
    for r in range(inner_permutation_rounds):
        c = np.empty(5, np.uint64)
        d = np.empty(5, np.uint64)
        b = np.empty(25, np.uint64)

        for x in range(5):
            c[x] = state_64[x] ^ state_64[x + 5] ^ state_64[x + 10] ^ state_64[x + 15] ^ state_64[x + 20] ^ harmonizer_64[1 + r % 24]

        for x in range(5):
            d[x] = c[(x + 4) % 5] ^ rotl_64(c[(x + 1) % 5], np.uint32(1))

            for y in range(5):
                state_64[x + 5*y] = (state_64[x + 5*y] ^ d[x]) & MASK64

        for x in range(5):
            for y in range(5):
                b[y + 5 * ((2*x + 3*y) % 5)] = rotl_64(state_64[x + 5*y], PERMUTATION_ROT[x * 5 + y])

        for x in range(5):
            for y in range(5):
                state_64[x + 5*y] = (b[x + 5*y] ^ ((~b[(x + 1) % 5 + 5*y]) & b[(x+2)%5 + 5*y])) & MASK64

        state_64[0] = (state_64[0] ^ PERMUTATION_RC[r % 24]) & MASK64

    for i in range(25):
        state[i * 2] = state_64[i] & MASK32
        state[i * 2 + 1] = state_64[i] >> 32

def compression_inner(part_a: NDArray[np.uint32], part_b: NDArray[np.uint32], round: int):
	z1 = zeta0(part_a[3])
	c = alpha0(part_a[3], part_a[4], part_a[5])

	t1 = np.add(part_a[7], np.add(z1, np.add(c, np.add(COMPRESSION_CONSTANTS[round % 64], part_b[round % 8], dtype=np.uint32), dtype=np.uint32), dtype=np.uint32), dtype=np.uint32)

	z0 = zeta1(part_a[7])
	d = beta(part_a[0], part_a[1], part_a[2])

	t2 = np.add(z0, d, dtype=np.uint32)

	part_a[7] = part_a[6]
	part_a[6] = part_a[5]
	part_a[5] = part_a[4]
	part_a[4] = np.add(d, t1, dtype=np.uint32)
	part_a[3] = part_a[2]
	part_a[2] = part_a[1]
	part_a[1] = part_a[0]
	part_a[0] = np.add(t1, t2, dtype=np.uint32)

def descent_compression_inner(lane_a: NDArray[np.uint32], lane_b: NDArray[np.uint32], k: int, p: int, r: int) -> NDArray[np.uint32]:
	curr_offset = k * 8
	prev_offset = p * 8

	z0 = zeta0(lane_a[curr_offset + 3])
	c = alpha0(lane_a[curr_offset + 3], lane_a[curr_offset + 4], lane_a[curr_offset + 5])

	t1 = np.add(lane_a[curr_offset + 7], np.add(z0, np.add(c, np.add(DESCENT_COMPRESSION_CONSTANTS[r % 8], lane_b[prev_offset + r % 8], dtype=np.uint32), dtype=np.uint), dtype=np.uint32))

	z1 = zeta1(np.add(lane_a[curr_offset + 7], lane_b[curr_offset + r % 8], dtype=np.uint32))

	d = beta(lane_a[curr_offset], lane_a[curr_offset + 1], lane_a[curr_offset + 2])

	t2 = np.add(z1, np.add(d, lane_b[curr_offset + (r + 1) % 8], dtype=np.uint32), dtype=np.uint32)

	lane_a[curr_offset + 7] = lane_a[curr_offset + 6]
	lane_a[curr_offset + 6] = lane_a[curr_offset + 5]
	lane_a[curr_offset + 5] = lane_a[curr_offset + 4]
	lane_a[curr_offset + 4] = np.add(d, t1, dtype=np.uint32) 
	lane_a[curr_offset + 3] = lane_a[curr_offset + 2]
	lane_a[curr_offset + 2] = lane_a[curr_offset + 1]
	lane_a[curr_offset + 1] = lane_a[curr_offset]
	lane_a[curr_offset] = np.add(t1, t2, dtype=np.uint32)

	return lane_a[curr_offset:curr_offset + 8]
