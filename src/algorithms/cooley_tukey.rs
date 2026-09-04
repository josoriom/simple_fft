use crate::algorithms::butterfly;
use crate::utilities::bit_reverse::{bit_reverse, count_bits};
use crate::utilities::twiddles::Twiddles;

/// Iterative decimation in time transform, in place.
/// Runs stages in pairs where it can, so most of the work reads the data once per two stages.
pub(crate) fn cooley_tukey(real: &mut [f64], imag: &mut [f64], twiddles: &Twiddles) {
    let size = real.len();
    if size < 2 {
        return;
    }
    bit_reverse(real);
    bit_reverse(imag);
    let stages = count_bits(size);
    let mut stage = 1;
    if stages % 2 == 1 {
        run_one_stage(real, imag, twiddles, stage);
        stage += 1;
    }
    while stage < stages {
        run_two_stages(real, imag, twiddles, stage);
        stage += 2;
    }
}

fn run_one_stage(real: &mut [f64], imag: &mut [f64], twiddles: &Twiddles, stage: u32) {
    let span = 1usize << stage;
    let turns = twiddles.get_stage(span / 2);
    for (block_real, block_imag) in real.chunks_exact_mut(span).zip(imag.chunks_exact_mut(span)) {
        butterfly::apply(block_real, block_imag, turns);
    }
}

fn run_two_stages(real: &mut [f64], imag: &mut [f64], twiddles: &Twiddles, stage: u32) {
    let span = 1usize << (stage + 1);
    let quarter = span / 4;
    let inner = twiddles.get_stage(quarter);
    let outer = twiddles.get_stage(quarter * 2);
    for (block_real, block_imag) in real.chunks_exact_mut(span).zip(imag.chunks_exact_mut(span)) {
        butterfly::radix4::apply(block_real, block_imag, inner, outer);
    }
}
