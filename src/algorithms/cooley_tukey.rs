use crate::algorithms::butterfly;
use crate::utilities::bit_reverse::{bit_reverse, count_bits};
use crate::utilities::twiddles::get_stage;
use crate::Complex;

/// Iterative decimation in time transform, in place.
/// Runs stages in pairs where it can, so most of the work reads the data once per two stages.
pub(crate) fn cooley_tukey(data: &mut [Complex], twiddles: &[Complex]) {
    let size = data.len();
    if size < 2 {
        return;
    }
    bit_reverse(data);
    let stages = count_bits(size);
    let mut stage = 1;
    if stages % 2 == 1 {
        run_one_stage(data, twiddles, stage);
        stage += 1;
    }
    while stage < stages {
        run_two_stages(data, twiddles, stage);
        stage += 2;
    }
}

fn run_one_stage(data: &mut [Complex], twiddles: &[Complex], stage: u32) {
    let span = 1 << stage;
    let half = span / 2;
    let stage_twiddles = get_stage(twiddles, half);
    for block in data.chunks_exact_mut(span) {
        let (near, far) = block.split_at_mut(half);
        butterfly::apply(near, far, stage_twiddles);
    }
}

fn run_two_stages(data: &mut [Complex], twiddles: &[Complex], stage: u32) {
    let span = 1 << (stage + 1);
    let quarter = span / 4;
    let inner = get_stage(twiddles, quarter);
    let outer = get_stage(twiddles, quarter * 2);
    for block in data.chunks_exact_mut(span) {
        butterfly::radix4::apply(block, inner, outer);
    }
}
