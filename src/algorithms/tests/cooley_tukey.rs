use crate::algorithms::cooley_tukey::cooley_tukey;
use crate::utilities::twiddles::Twiddles;
use crate::Complex;

fn run(values: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut real = values.to_vec();
    let mut imag = vec![0.0; values.len()];
    let twiddles = Twiddles::new(values.len());
    cooley_tukey(&mut real, &mut imag, &twiddles);
    (real, imag)
}

fn is_close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-12
}

#[test]
fn impulse_gives_all_ones() {
    let (real, imag) = run(&[1.0, 0.0, 0.0, 0.0]);
    for index in 0..4 {
        assert!(is_close(real[index], 1.0) && is_close(imag[index], 0.0));
    }
}

#[test]
fn constant_gives_only_first_bin() {
    let (real, imag) = run(&[1.0, 1.0, 1.0, 1.0]);
    assert!(is_close(real[0], 4.0));
    for index in 1..4 {
        assert!(is_close(real[index], 0.0) && is_close(imag[index], 0.0));
    }
}

#[test]
fn ramp_matches_hand_computed_dft() {
    let (real, imag) = run(&[1.0, 2.0, 3.0, 4.0]);
    assert!(is_close(real[0], 10.0) && is_close(imag[0], 0.0));
    assert!(is_close(real[1], -2.0) && is_close(imag[1], 2.0));
    assert!(is_close(real[2], -2.0) && is_close(imag[2], 0.0));
    assert!(is_close(real[3], -2.0) && is_close(imag[3], -2.0));
}

fn naive_dft(values: &[f64]) -> Vec<Complex> {
    let size = values.len();
    (0..size)
        .map(|bin| {
            let mut sum = Complex::zero();
            for (position, value) in values.iter().enumerate() {
                let angle = -std::f64::consts::TAU * bin as f64 * position as f64 / size as f64;
                sum = sum + Complex::from_angle(angle).scale(*value);
            }
            sum
        })
        .collect()
}

#[test]
fn matches_the_slow_dft_for_many_sizes() {
    for size in [8usize, 16, 32, 64, 128] {
        let values: Vec<f64> = (0..size).map(|i| (i as f64 * 0.7).sin() + 0.3).collect();
        let (real, imag) = run(&values);
        let slow = naive_dft(&values);
        for bin in 0..size {
            assert!(
                (real[bin] - slow[bin].real).abs() < 1e-11,
                "size {} bin {}",
                size,
                bin
            );
            assert!(
                (imag[bin] - slow[bin].imag).abs() < 1e-11,
                "size {} bin {}",
                size,
                bin
            );
        }
    }
}
