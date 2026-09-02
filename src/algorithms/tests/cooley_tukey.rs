use crate::algorithms::cooley_tukey::cooley_tukey;
use crate::utilities::twiddles::get_twiddles;
use crate::Complex;

fn run(values: &[f64]) -> Vec<Complex> {
    let mut data: Vec<Complex> = values.iter().map(|v| Complex::new(*v, 0.0)).collect();
    let twiddles = get_twiddles(data.len());
    cooley_tukey(&mut data, &twiddles);
    data
}

fn is_close(left: Complex, right: Complex) -> bool {
    (left.real - right.real).abs() < 1e-12 && (left.imag - right.imag).abs() < 1e-12
}

#[test]
fn impulse_gives_all_ones() {
    let output = run(&[1.0, 0.0, 0.0, 0.0]);
    for point in output {
        assert!(is_close(point, Complex::new(1.0, 0.0)), "{:?}", point);
    }
}

#[test]
fn constant_gives_only_first_bin() {
    let output = run(&[1.0, 1.0, 1.0, 1.0]);
    assert!(is_close(output[0], Complex::new(4.0, 0.0)));
    for point in &output[1..] {
        assert!(is_close(*point, Complex::zero()), "{:?}", point);
    }
}

#[test]
fn ramp_matches_hand_computed_dft() {
    let output = run(&[1.0, 2.0, 3.0, 4.0]);
    assert!(is_close(output[0], Complex::new(10.0, 0.0)), "{:?}", output[0]);
    assert!(is_close(output[1], Complex::new(-2.0, 2.0)), "{:?}", output[1]);
    assert!(is_close(output[2], Complex::new(-2.0, 0.0)), "{:?}", output[2]);
    assert!(is_close(output[3], Complex::new(-2.0, -2.0)), "{:?}", output[3]);
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
fn matches_the_slow_dft_for_eight_and_sixteen() {
    for size in [8usize, 16] {
        let values: Vec<f64> = (0..size).map(|i| (i as f64 * 0.7).sin() + 0.3).collect();
        let fast = run(&values);
        let slow = naive_dft(&values);
        for (bin, (a, b)) in fast.iter().zip(slow.iter()).enumerate() {
            assert!((a.real - b.real).abs() < 1e-11, "size {} bin {}", size, bin);
            assert!((a.imag - b.imag).abs() < 1e-11, "size {} bin {}", size, bin);
        }
    }
}
