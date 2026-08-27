pub trait TundraImplementation {
    fn expand_generate(&self, n: usize, o: &[u8], w: usize) -> u8;

    fn permutation_inner(
        &self,
        state: &mut [u32],
        harmonizer: &[u32],
        inner_permutation_rounds: usize,
    );

    fn compress_inner(&self, part_a: &mut [u32], part_b: &[u32], round: usize);

    fn descent_compression_inner(
        &self,
        lane_a: &mut [u32],
        part_b: &[u32],
        k: usize,
        p: usize,
        r: usize,
    ) -> Vec<u32>;
}
