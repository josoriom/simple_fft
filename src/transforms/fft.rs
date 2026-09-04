use crate::transforms::plan::Plan;
use crate::Data;

/// Discrete Fourier transform, X[k] = sum over n of x[n] e^(-2*pi*i*k*n/size).
/// Size must be a power of two. Build a `Plan` instead when transforming the same size often.
pub fn fft(input: Data) -> Data {
    Plan::new(input.len()).fft(input)
}

/// Inverse transform through the conjugate identity, x = conj(fft(conj(X))) / size.
/// Size must be a power of two. Build a `Plan` instead when transforming the same size often.
pub fn ifft(input: Data) -> Data {
    Plan::new(input.len()).ifft(input)
}
