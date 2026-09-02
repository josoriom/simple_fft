use crate::structs::Complex;

fn is_close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-12
}

#[test]
fn adds_both_parts() {
    let result = Complex::new(1.0, 2.0) + Complex::new(10.0, 20.0);
    assert_eq!(result, Complex::new(11.0, 22.0));
}

#[test]
fn subtracts_both_parts() {
    let result = Complex::new(10.0, 20.0) - Complex::new(1.0, 2.0);
    assert_eq!(result, Complex::new(9.0, 18.0));
}

#[test]
fn multiplies_using_the_rule() {
    let result = Complex::new(1.0, 2.0) * Complex::new(3.0, 4.0);
    assert_eq!(result, Complex::new(-5.0, 10.0));
}

#[test]
fn multiplying_i_by_i_gives_minus_one() {
    let i = Complex::new(0.0, 1.0);
    assert_eq!(i * i, Complex::new(-1.0, 0.0));
}

#[test]
fn angle_of_zero_points_right() {
    let point = Complex::from_angle(0.0);
    assert!(is_close(point.real, 1.0));
    assert!(is_close(point.imag, 0.0));
}

#[test]
fn half_turn_points_left() {
    let point = Complex::from_angle(std::f64::consts::PI);
    assert!(is_close(point.real, -1.0));
    assert!(is_close(point.imag, 0.0));
}

#[test]
fn size_of_three_four_is_five() {
    assert!(is_close(Complex::new(3.0, 4.0).size(), 5.0));
}
