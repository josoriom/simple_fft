use crate::structs::{Complex, Input};

#[test]
fn real_input_reports_its_length() {
    let input = Input::Real(vec![1.0, 2.0, 3.0]);
    assert_eq!(input.len(), 3);
}

#[test]
fn complex_input_reports_its_length() {
    let input = Input::Complex(vec![Complex::new(1.0, 1.0), Complex::new(2.0, 2.0)]);
    assert_eq!(input.len(), 2);
}

#[test]
fn empty_input_is_empty() {
    assert!(Input::Real(vec![]).is_empty());
}

#[test]
fn real_values_get_a_zero_imag_part() {
    let points = Input::Real(vec![4.0, 5.0]).into_points();
    assert_eq!(points, vec![Complex::new(4.0, 0.0), Complex::new(5.0, 0.0)]);
}

#[test]
fn complex_values_pass_through_unchanged() {
    let points = vec![Complex::new(1.0, -2.0), Complex::new(0.0, 3.0)];
    assert_eq!(Input::Complex(points.clone()).into_points(), points);
}

#[test]
fn a_list_of_real_numbers_converts_into_input() {
    let input: Input = vec![7.0, 8.0].into();
    assert_eq!(input, Input::Real(vec![7.0, 8.0]));
}

#[test]
fn a_list_of_points_converts_into_input() {
    let points = vec![Complex::new(1.0, 1.0)];
    let input: Input = points.clone().into();
    assert_eq!(input, Input::Complex(points));
}
