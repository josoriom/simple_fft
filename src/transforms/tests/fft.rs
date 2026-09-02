use crate::transforms::fft::{fft, ifft};
use crate::{Complex, Input};

fn is_close(left: Complex, right: Complex) -> bool {
    (left.real - right.real).abs() < 1e-12 && (left.imag - right.imag).abs() < 1e-12
}

#[test]
fn constant_signal_has_only_a_first_bin() {
    let output = fft(Input::Real(vec![1.0, 1.0, 1.0, 1.0]));
    assert!(is_close(output[0], Complex::new(4.0, 0.0)));
    for point in &output[1..] {
        assert!(is_close(*point, Complex::zero()));
    }
}

#[test]
fn a_ramp_matches_the_hand_computed_transform() {
    let output = fft(Input::Real(vec![1.0, 2.0, 3.0, 4.0]));
    assert!(is_close(output[0], Complex::new(10.0, 0.0)));
    assert!(is_close(output[1], Complex::new(-2.0, 2.0)));
    assert!(is_close(output[2], Complex::new(-2.0, 0.0)));
    assert!(is_close(output[3], Complex::new(-2.0, -2.0)));
}

#[test]
fn inverse_undoes_the_transform() {
    let values: Vec<f64> = (0..64).map(|i| (i as f64 * 0.37).sin() * 3.0 - 1.0).collect();
    let restored = ifft(Input::Complex(fft(Input::Real(values.clone()))));
    for (value, point) in values.iter().zip(restored) {
        assert!(is_close(point, Complex::new(*value, 0.0)), "{:?}", point);
    }
}

#[test]
fn inverse_of_a_single_bin_is_a_constant() {
    let mut spectrum = vec![Complex::zero(); 8];
    spectrum[0] = Complex::new(8.0, 0.0);
    for point in ifft(Input::Complex(spectrum)) {
        assert!(is_close(point, Complex::new(1.0, 0.0)));
    }
}

#[test]
#[should_panic(expected = "Size must be a power of two.")]
fn rejects_a_length_that_is_not_a_power_of_two() {
    fft(Input::Real(vec![1.0, 2.0, 3.0]));
}
