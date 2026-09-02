use crate::transforms::plan::Plan;
use crate::{Complex, Input};

/// Circular convolution through the convolution theorem, ifft(fft(left) * fft(right)).
/// Both inputs must have the same length and be a power of two.
pub fn convolution(left: Input, right: Input) -> Vec<Complex> {
    combine(left, right, |a, b| a * b)
}

/// Circular deconvolution, the inverse of convolution, ifft(fft(left) / fft(right)).
/// Both inputs must have the same length and be a power of two.
pub fn deconvolution(left: Input, right: Input) -> Vec<Complex> {
    combine(left, right, |a, b| a / b)
}

fn combine(left: Input, right: Input, operation: fn(Complex, Complex) -> Complex) -> Vec<Complex> {
    assert_eq!(
        left.len(),
        right.len(),
        "Both inputs must have the same length."
    );
    let plan = Plan::new(left.len());
    let paired: Vec<Complex> = plan
        .fft(left)
        .into_iter()
        .zip(plan.fft(right))
        .map(|(a, b)| operation(a, b))
        .collect();
    plan.ifft(Input::Complex(paired))
}
