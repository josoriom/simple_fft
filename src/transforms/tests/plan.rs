use crate::transforms::plan::Plan;
use crate::{fft, Complex, Input};

fn is_close(left: Complex, right: Complex) -> bool {
    (left.real - right.real).abs() < 1e-12 && (left.imag - right.imag).abs() < 1e-12
}

#[test]
fn reports_the_size_it_was_built_for() {
    assert_eq!(Plan::new(64).size(), 64);
}

#[test]
fn gives_the_same_answer_as_the_plain_call() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let plan = Plan::new(values.len());
    for point in plan
        .fft(Input::Real(values.clone()))
        .iter()
        .zip(fft(Input::Real(values)))
    {
        assert!(is_close(*point.0, point.1));
    }
}

#[test]
fn one_plan_serves_many_calls() {
    let plan = Plan::new(16);
    for round in 0..5 {
        let values: Vec<f64> = (0..16).map(|i| (i + round) as f64).collect();
        let restored = plan.ifft(Input::Complex(plan.fft(Input::Real(values.clone()))));
        for (point, value) in restored.iter().zip(values) {
            assert!(is_close(*point, Complex::new(value, 0.0)), "{:?}", point);
        }
    }
}

#[test]
#[should_panic(expected = "Input must match the plan size.")]
fn rejects_input_of_another_size() {
    Plan::new(8).fft(Input::Real(vec![1.0, 2.0, 3.0, 4.0]));
}
