use crate::Complex;
use std::f64::consts::TAU;

/// Twiddle factors for every stage, laid out back to back so each stage reads a contiguous slice.
/// Stage with span 2^s holds W = e^(-2*pi*i*k/span) for k in 0..span/2, starting at span/2 - 1.
/// Total length is size - 1.
pub(crate) fn get_twiddles(size: usize) -> Vec<Complex> {
    if size < 2 {
        return Vec::new();
    }
    let mut twiddles = Vec::with_capacity(size - 1);
    let mut span = 2;
    while span <= size {
        let half = span / 2;
        for index in 0..half {
            twiddles.push(Complex::from_angle(-TAU * index as f64 / span as f64));
        }
        span *= 2;
    }
    twiddles
}

/// Slice of factors belonging to the stage whose half span is `half`.
#[inline]
pub(crate) fn get_stage(twiddles: &[Complex], half: usize) -> &[Complex] {
    &twiddles[half - 1..half * 2 - 1]
}
