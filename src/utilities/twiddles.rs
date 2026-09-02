use crate::Complex;
use std::f64::consts::TAU;

/// Twiddle factors W = e^(-2*pi*i*index/size) for index in 0..size/2, on the unit circle.
pub(crate) fn get_twiddles(size: usize) -> Vec<Complex> {
    let count = size / 2;
    let mut twiddles = Vec::with_capacity(count);
    for index in 0..count {
        twiddles.push(Complex::from_angle(-TAU * index as f64 / size as f64));
    }
    twiddles
}
