//!
//! The Nessak implementation of the Tundra structure.
//!

use alloc::vec::Vec;

use crate::tundra::{Tundra, TundraImplementation};

pub const INNER_PERMUTATION_ROT: [u32; 25] = [
    0, 36, 3, 41, 18, 1, 44, 10, 45, 2, 62, 6, 43, 15, 61, 28, 55, 25, 21, 56, 27, 20, 39, 8, 14,
];

pub const INNER_PERMUTATION_RC: [u64; 24] = [
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
    0x8000000080008008,
];

pub const COMPRESSION_CONSTANTS: [u32; 64] = [
    0xCBBB9D5D, 0x629A292A, 0x9159015A, 0x152FECD8, 0x67332667, 0x8EB44A87, 0xDB0C2E0D, 0x47B5481D,
    0xAE5F9156, 0xCF6C85D3, 0x2F73477D, 0x6D1826CA, 0x8B43D457, 0xE360B596, 0x1C456002, 0x6F196331,
    0xD94EBEB1, 0x0CC4A611, 0x261DC1F2, 0x5815A7BE, 0x70B7ED67, 0xA1513C69, 0x44F93635, 0x720DCDFD,
    0xB467369E, 0xCA320B75, 0x34E0D42E, 0x49C7D9BD, 0x87ABB9F2, 0xC463A2FC, 0xEC3FC3F3, 0x27277F6D,
    0x610BEBF2, 0x7420B49E, 0xD1FD8A33, 0xE4773594, 0x092197F6, 0x1B530C95, 0x869D6342, 0xEEE52E4F,
    0x11076689, 0x21FBA37B, 0x43AB9FB6, 0x75A9F91D, 0x86305019, 0xD7CD8173, 0x07FE00FF, 0x379F513F,
    0x66B651A8, 0x764AB842, 0xA4B06BE1, 0xC3578C15, 0xD2962A53, 0x1E039F40, 0x857B7BEE, 0xA29BF2DE,
    0xB11A32E8, 0xCDF34E80, 0x31830426, 0x5B89092B, 0xA0C06A13, 0xAE79842F, 0xC9CDA689, 0xF281F239,
];

pub const DESCENT_COMPRESSION_CONSTANTS: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

pub fn rotr(x: u32, n: u32) -> u32 {
    return (x >> n) | (x << (32 - n));
}

pub fn zeta0(x: u32) -> u32 {
    return rotr(x, 2) ^ rotr(x, 7) ^ rotr(x, 17) ^ x.wrapping_shr(3);
}

pub fn zeta1(x: u32) -> u32 {
    return rotr(x, 17) ^ rotr(x, 19) ^ x.wrapping_shr(11);
}

pub fn alpha0(x: u32, y: u32, z: u32) -> u32 {
    return (x & rotr(y, 3)) ^ ((!x) & z);
}

pub fn beta(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) ^ (z & x) ^ (y & z);
}

pub fn rotl_64(x: u64, n: u32) -> u64 {
    return (x << n) | (x.wrapping_shr(64 - n));
}

pub struct NessakTundraImplementation {}

impl TundraImplementation for NessakTundraImplementation {
    const MINIMUM_EXPANSION_LEN: usize = 56;
    const PART_SIZE: usize = 8;
    const PERMUTATION_MUL_SIZE: usize = 50;

    fn expand_generate(n: usize, o: &[u32], w: usize) -> u32 {
        o[n - w]
            .wrapping_add(zeta0(o[n - 14]))
            .wrapping_add(zeta1(o[n - 2]))
            .wrapping_add(o[n - 6])
    }

    fn permutation_inner(state: &mut [u32], harmonizer: &[u32], inner_permutation_rounds: usize) {
        let mut state_64: [u64; 25] = [0; 25];
        let mut harmonizer_64: [u64; 25] = [0; 25];

        // Unpack the 64-bit number state
        for i in 0..25 {
            state_64[i] = u64::from(state[i * 2]) | u64::from(state[i * 2 + 1]) << 32;
            harmonizer_64[i] =
                u64::from(harmonizer[i * 2]) | u64::from(harmonizer[i * 2 + 1]) << 32;
        }

        // initialize arrays here so we don't need to destroy them
        let mut c: [u64; 5] = [0; 5];
        let mut k: [u64; 5] = [0; 5];
        let mut b: [u64; 25] = [0; 25];

        for r in 0..inner_permutation_rounds {
            for x in 0..5 {
                c[x] = state_64[x]
                    ^ state_64[x + 5]
                    ^ state_64[x + 10]
                    ^ state_64[x + 15]
                    ^ state_64[x + 20]
                    ^ harmonizer_64[1 + r % 24]
            }

            for x in 0..5 {
                k[x] = c[(x + 4) % 5] ^ rotl_64(c[(x + 1) % 5], 1);

                for y in 0..5 {
                    state_64[x + 5 * y] = state_64[x + 5 * y] ^ k[x];
                }
            }

            for i in 0..25 {
                let x = i / 5;
                let y = i % 5;

                b[y + 5 * ((2 * x + 3 * y) % 5)] =
                    rotl_64(state_64[x + 5 * y], INNER_PERMUTATION_ROT[5 * x + y])
            }

            for i in 0..25 {
                let x = i / 5;
                let y = i % 5;

                state_64[x + 5 * y] =
                    b[x + 5 * y] ^ ((!b[(x + 1) % 5 + 5 * y]) & b[(x + 2) % 5 + 5 * y]);
            }

            state_64[0] ^= INNER_PERMUTATION_RC[r % 24];
        }

        // Repack the 64-bit number state
        for i in 0..25 {
            state[i * 2] = (state_64[i] & (1 << 32) - 1) as u32;
            state[i * 2 + 1] = (state_64[i] >> 32) as u32;
        }
    }

    fn compress_inner(part_a: &mut [u32], part_b: &[u32], round: usize) {
        let z1 = zeta0(part_a[3]);
        let c = alpha0(part_a[3], part_a[4], part_a[5]);

        let t1 = part_a[7]
            .wrapping_add(z1)
            .wrapping_add(c)
            .wrapping_add(COMPRESSION_CONSTANTS[round % 64])
            .wrapping_add(part_b[round % 8]);

        let z0 = zeta1(part_a[7]);
        let d = beta(part_a[0], part_a[1], part_a[2]);

        let t2 = z0.wrapping_add(d);

        part_a[7] = part_a[6];
        part_a[6] = part_a[5];
        part_a[5] = part_a[4];
        part_a[4] = d.wrapping_add(t1);
        part_a[3] = part_a[2];
        part_a[2] = part_a[1];
        part_a[1] = part_a[0];
        part_a[0] = t1.wrapping_add(t2);
    }

    fn descent_compression_inner(lane_a: &mut [u32], lane_b: &[u32], k: usize, p: usize, r: usize) {
        let curr_offset = k * 8;
        let prev_offset = p * 8;

        let z0 = zeta0(lane_a[curr_offset + 3]);
        let c = alpha0(
            lane_a[curr_offset + 3],
            lane_a[curr_offset + 4],
            lane_a[curr_offset + 5],
        );

        let t1 = lane_a[curr_offset + 7]
            .wrapping_add(z0)
            .wrapping_add(c)
            .wrapping_add(DESCENT_COMPRESSION_CONSTANTS[r % 8])
            .wrapping_add(lane_b[prev_offset + r % 8]);

        let z1 = zeta1(lane_a[curr_offset + 7].wrapping_add(lane_b[curr_offset + r % 8]));
        let d = beta(
            lane_a[curr_offset],
            lane_a[curr_offset + 1],
            lane_a[curr_offset + 2],
        );

        let t2 = z1
            .wrapping_add(d)
            .wrapping_add(lane_b[curr_offset + (r + 1) % 8]);

        lane_a[curr_offset + 7] = lane_a[curr_offset + 6];
        lane_a[curr_offset + 6] = lane_a[curr_offset + 5];
        lane_a[curr_offset + 5] = lane_a[curr_offset + 4];
        lane_a[curr_offset + 4] = d.wrapping_add(t1);
        lane_a[curr_offset + 3] = lane_a[curr_offset + 2];
        lane_a[curr_offset + 2] = lane_a[curr_offset + 1];
        lane_a[curr_offset + 1] = lane_a[curr_offset];
        lane_a[curr_offset] = t1.wrapping_add(t2);
    }
}

impl NessakTundraImplementation {
    pub fn k2048_2048(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 2048, 2048, 64, 8, 24, 4, 1)
    }

    pub fn k1024_1024(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 1024, 1024, 64, 8, 24, 4, 1)
    }

    pub fn k512_512(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 512, 512, 64, 8, 24, 4, 1)
    }

    pub fn k256_256(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 256, 256, 64, 8, 24, 4, 1)
    }

    pub fn k256_128(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 128, 256, 64, 8, 24, 4, 1)
    }

    pub fn k256_64(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 64, 256, 64, 8, 24, 4, 1)
    }

    pub fn k256_32(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 32, 256, 64, 8, 24, 4, 1)
    }

    pub fn k256_16(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 16, 256, 64, 8, 24, 4, 1)
    }

    pub fn k4096_2048(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 2048, 4096, 64, 16, 24, 16, 2)
    }

    pub fn k2048_1024(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 1024, 2048, 64, 16, 24, 16, 2)
    }

    pub fn k1024_512(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 512, 1024, 64, 16, 24, 16, 2)
    }

    pub fn k512_256(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 256, 512, 64, 16, 24, 16, 2)
    }

    pub fn k512_128(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 128, 512, 64, 16, 24, 16, 2)
    }

    pub fn k512_64(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 64, 512, 64, 16, 24, 16, 2)
    }

    pub fn k512_32(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 32, 512, 64, 16, 24, 16, 2)
    }

    pub fn k512_16(input: &[u8]) -> Vec<u8> {
        Self::produce_hash(input, 16, 512, 64, 16, 24, 16, 2)
    }
}
