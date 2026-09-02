use crate::transforms::convolution::{convolution, deconvolution};
use crate::{Complex, Input};

fn is_close(left: Complex, right: f64) -> bool {
    (left.real - right).abs() < 1e-10 && left.imag.abs() < 1e-10
}

fn naive_circular_convolution(left: &[f64], right: &[f64]) -> Vec<f64> {
    let size = left.len();
    (0..size)
        .map(|position| {
            (0..size)
                .map(|shift| left[shift] * right[(position + size - shift) % size])
                .sum()
        })
        .collect()
}

#[test]
fn convolving_with_a_delta_returns_the_signal() {
    let signal = vec![5.0, 1.0, -2.0, 4.0];
    let delta = vec![1.0, 0.0, 0.0, 0.0];
    let output = convolution(Input::Real(signal.clone()), Input::Real(delta));
    for (point, value) in output.iter().zip(signal) {
        assert!(is_close(*point, value), "{:?}", point);
    }
}

#[test]
fn matches_the_slow_circular_convolution() {
    let left = vec![1.0, 2.0, 3.0, 4.0, 0.0, -1.0, 2.0, 1.0];
    let right = vec![0.5, 1.0, -1.0, 2.0, 1.0, 0.0, 3.0, -2.0];
    let fast = convolution(Input::Real(left.clone()), Input::Real(right.clone()));
    let slow = naive_circular_convolution(&left, &right);
    for (point, value) in fast.iter().zip(slow) {
        assert!(is_close(*point, value), "{:?} vs {}", point, value);
    }
}

#[test]
fn deconvolution_undoes_convolution() {
    let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let filter = vec![1.0, 0.5, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0];
    let blurred = convolution(Input::Real(signal.clone()), Input::Real(filter.clone()));
    let restored = deconvolution(Input::Complex(blurred), Input::Real(filter));
    for (point, value) in restored.iter().zip(signal) {
        assert!(is_close(*point, value), "{:?} vs {}", point, value);
    }
}

#[test]
#[should_panic(expected = "Both inputs must have the same length.")]
fn rejects_inputs_of_different_lengths() {
    convolution(Input::Real(vec![1.0, 2.0]), Input::Real(vec![1.0, 2.0, 3.0, 4.0]));
}
