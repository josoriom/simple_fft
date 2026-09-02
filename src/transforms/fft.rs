use crate::algorithms::cooley_tukey::cooley_tukey;
use crate::utilities::twiddles::get_twiddles;
use crate::{Complex, Input};

/// Discrete Fourier transform, X[k] = sum over n of x[n] e^(-2*pi*i*k*n/size).
/// Size must be a power of two.
pub fn fft(input: Input) -> Vec<Complex> {
    let mut data = input.into_points();
    let twiddles = get_twiddles(data.len());
    cooley_tukey(&mut data, &twiddles);
    data
}

/// Inverse transform through the conjugate identity, x = conj(fft(conj(X))) / size.
/// Size must be a power of two.
pub fn ifft(input: Input) -> Vec<Complex> {
    let mut data = input.into_points();
    let size = data.len();
    if size == 0 {
        return data;
    }
    for point in data.iter_mut() {
        *point = point.flip_sign_of_imag();
    }
    let twiddles = get_twiddles(size);
    cooley_tukey(&mut data, &twiddles);
    let factor = 1.0 / size as f64;
    for point in data.iter_mut() {
        *point = point.flip_sign_of_imag().scale(factor);
    }
    data
}
