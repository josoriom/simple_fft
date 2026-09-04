use crate::transforms::plan::Plan;
use crate::{Complex, Data};

/// Circular convolution through the convolution theorem, ifft(fft(left) * fft(right)).
/// Both inputs must have the same length and be a power of two.
pub fn convolution(left: Data, right: Data) -> Data {
    combine(left, right, |a, b| a * b)
}

/// Circular deconvolution, the inverse of convolution, ifft(fft(left) / fft(right)).
/// Both inputs must have the same length and be a power of two.
pub fn deconvolution(left: Data, right: Data) -> Data {
    combine(left, right, |a, b| a / b)
}

fn combine(left: Data, right: Data, operation: fn(Complex, Complex) -> Complex) -> Data {
    assert_eq!(
        left.len(),
        right.len(),
        "Both inputs must have the same length."
    );
    let plan = Plan::new(left.len());
    let mut first = plan.fft(left);
    let second = plan.fft(right);
    for index in 0..first.len() {
        let paired = operation(
            Complex::new(first.real[index], first.imag[index]),
            Complex::new(second.real[index], second.imag[index]),
        );
        first.real[index] = paired.real;
        first.imag[index] = paired.imag;
    }
    plan.ifft(first)
}
