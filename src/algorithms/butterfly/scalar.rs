use crate::utilities::twiddles::Turns;
use crate::Complex;

/// One butterfly per step over a block held as two arrays.
/// near = near + turn * far, far = near - turn * far.
pub(crate) fn apply(real: &mut [f64], imag: &mut [f64], turns: Turns) {
    let half = real.len() / 2;
    for index in 0..half {
        let far = index + half;
        let top = Complex::new(real[index], imag[index]);
        let turn = Complex::new(turns.real[index], turns.imag[index]);
        let bottom = turn * Complex::new(real[far], imag[far]);
        let sum = top + bottom;
        let difference = top - bottom;
        real[index] = sum.real;
        imag[index] = sum.imag;
        real[far] = difference.real;
        imag[far] = difference.imag;
    }
}
