use crate::transforms::fft::{fft, ifft};
use crate::Data;

fn from_real(values: Vec<f64>) -> Data {
    let imag = vec![0.0; values.len()];
    Data { real: values, imag }
}

fn is_close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-12
}

#[test]
fn constant_signal_has_only_a_first_bin() {
    let output = fft(from_real(vec![1.0, 1.0, 1.0, 1.0]));
    assert!(is_close(output.real[0], 4.0));
    for index in 1..4 {
        assert!(is_close(output.real[index], 0.0) && is_close(output.imag[index], 0.0));
    }
}

#[test]
fn a_ramp_matches_the_hand_computed_transform() {
    let output = fft(from_real(vec![1.0, 2.0, 3.0, 4.0]));
    assert_eq!(output.real.len(), 4);
    assert!(is_close(output.real[0], 10.0) && is_close(output.imag[0], 0.0));
    assert!(is_close(output.real[1], -2.0) && is_close(output.imag[1], 2.0));
    assert!(is_close(output.real[2], -2.0) && is_close(output.imag[2], 0.0));
    assert!(is_close(output.real[3], -2.0) && is_close(output.imag[3], -2.0));
}

#[test]
fn inverse_undoes_the_transform() {
    let values: Vec<f64> = (0..64)
        .map(|i| (i as f64 * 0.37).sin() * 3.0 - 1.0)
        .collect();
    let restored = ifft(fft(from_real(values.clone())));
    for (index, value) in values.iter().enumerate() {
        assert!(is_close(restored.real[index], *value), "{}", index);
        assert!(is_close(restored.imag[index], 0.0), "{}", index);
    }
}

#[test]
fn inverse_of_a_single_bin_is_a_constant() {
    let mut spectrum = from_real(vec![0.0; 8]);
    spectrum.real[0] = 8.0;
    let output = ifft(spectrum);
    for index in 0..8 {
        assert!(is_close(output.real[index], 1.0) && is_close(output.imag[index], 0.0));
    }
}

#[test]
fn complex_input_survives_a_round_trip() {
    let real: Vec<f64> = (0..32).map(|i| (i as f64 * 0.2).sin()).collect();
    let imag: Vec<f64> = (0..32).map(|i| (i as f64 * 0.3).cos()).collect();
    let restored = ifft(fft(Data {
        real: real.clone(),
        imag: imag.clone(),
    }));
    for index in 0..32 {
        assert!(
            is_close(restored.real[index], real[index]),
            "real {}",
            index
        );
        assert!(
            is_close(restored.imag[index], imag[index]),
            "imag {}",
            index
        );
    }
}

#[test]
#[should_panic(expected = "Size must be a power of two.")]
fn rejects_a_length_that_is_not_a_power_of_two() {
    fft(from_real(vec![1.0, 2.0, 3.0]));
}

#[test]
#[should_panic(expected = "Real and imaginary parts must have the same length.")]
fn rejects_parts_of_different_lengths() {
    fft(Data {
        real: vec![1.0, 2.0, 3.0, 4.0],
        imag: vec![0.0],
    });
}
