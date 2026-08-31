pub trait TundraPreset {
    const DIGEST_SIZE: usize;
    const LANE_SIZE: usize;
    const COMPRESSION_ROUNDS: usize;
    const DESCENT_COMPRESSION_ROUNDS: usize;
    const INNER_PERMUTATION_ROUNDS: usize;
    const OUTER_PERMUTATION_ROUNDS: usize;
    const INTERNAL_STATE_MINIMUM_LENGTH_MULTIPLIER: usize;
}
