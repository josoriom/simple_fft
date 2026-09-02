use crate::transforms::fft::{fft, ifft};
use crate::{Complex, Input};

/// Circular convolution through the convolution theorem, ifft(fft(left) * fft(right)).
/// Both inputs must have the same length and be a power of two.
pub fn convolution(left: Input, right: Input) -> Vec<Complex> {
    let product = combine(left, right, |a, b| a * b);
    ifft(Input::Complex(product))
}

/// Circular deconvolution, the inverse of convolution, ifft(fft(left) / fft(right)).
/// Both inputs must have the same length and be a power of two.
pub fn deconvolution(left: Input, right: Input) -> Vec<Complex> {
    let quotient = combine(left, right, |a, b| a / b);
    ifft(Input::Complex(quotient))
}

fn combine(
    left: Input,
    right: Input,
    operation: fn(Complex, Complex) -> Complex,
) -> Vec<Complex> {
    assert_eq!(left.len(), right.len(), "Both inputs must have the same length.");
    let left_spectrum = fft(left);
    let right_spectrum = fft(right);
    left_spectrum
        .into_iter()
        .zip(right_spectrum)
        .map(|(a, b)| operation(a, b))
        .collect()
}
