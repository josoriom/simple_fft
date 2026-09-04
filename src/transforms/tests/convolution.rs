use crate::transforms::convolution::{convolution, deconvolution};
use crate::Data;

fn from_real(values: Vec<f64>) -> Data {
    let imag = vec![0.0; values.len()];
    Data { real: values, imag }
}

fn is_close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-10
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
    let output = convolution(
        from_real(signal.clone()),
        from_real(vec![1.0, 0.0, 0.0, 0.0]),
    );
    for (index, value) in signal.iter().enumerate() {
        assert!(is_close(output.real[index], *value), "{}", index);
    }
}

#[test]
fn matches_the_slow_circular_convolution() {
    let left = vec![1.0, 2.0, 3.0, 4.0, 0.0, -1.0, 2.0, 1.0];
    let right = vec![0.5, 1.0, -1.0, 2.0, 1.0, 0.0, 3.0, -2.0];
    let fast = convolution(from_real(left.clone()), from_real(right.clone()));
    let slow = naive_circular_convolution(&left, &right);
    for (index, value) in slow.iter().enumerate() {
        assert!(is_close(fast.real[index], *value), "{}", index);
    }
}

#[test]
fn deconvolution_undoes_convolution() {
    let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let filter = vec![1.0, 0.5, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0];
    let blurred = convolution(from_real(signal.clone()), from_real(filter.clone()));
    let restored = deconvolution(blurred, from_real(filter));
    for (index, value) in signal.iter().enumerate() {
        assert!(is_close(restored.real[index], *value), "{}", index);
    }
}

#[test]
#[should_panic(expected = "Both inputs must have the same length.")]
fn rejects_inputs_of_different_lengths() {
    convolution(
        from_real(vec![1.0, 2.0]),
        from_real(vec![1.0, 2.0, 3.0, 4.0]),
    );
}
