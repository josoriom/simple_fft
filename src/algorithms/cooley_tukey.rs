use crate::utilities::bit_reverse::{bit_reverse, count_bits};
use crate::Complex;

/// Iterative radix-2 decimation in time transform, in place.
/// Runs log2(size) stages of butterflies over data already in bit reversed order.
pub(crate) fn cooley_tukey(data: &mut [Complex], twiddles: &[Complex]) {
    let size = data.len();
    if size < 2 {
        return;
    }
    bit_reverse(data);
    for stage in 1..=count_bits(size) {
        let span = 1 << stage;
        let half = span / 2;
        let step = size / span;
        for block_start in (0..size).step_by(span) {
            for offset in 0..half {
                let near = block_start + offset;
                let far = near + half;
                let top = data[near];
                let bottom = twiddles[offset * step] * data[far];
                data[near] = top + bottom;
                data[far] = top - bottom;
            }
        }
    }
}
