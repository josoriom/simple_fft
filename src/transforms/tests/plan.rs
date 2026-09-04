use crate::transforms::plan::Plan;
use crate::{fft, Data};

fn from_real(values: Vec<f64>) -> Data {
    let imag = vec![0.0; values.len()];
    Data { real: values, imag }
}

fn is_close(left: f64, right: f64) -> bool {
    (left - right).abs() < 1e-12
}

#[test]
fn reports_the_size_it_was_built_for() {
    assert_eq!(Plan::new(64).size(), 64);
}

#[test]
fn gives_the_same_answer_as_the_plain_call() {
    let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let planned = Plan::new(values.len()).fft(from_real(values.clone()));
    let plain = fft(from_real(values));
    assert_eq!(planned, plain);
}

#[test]
fn one_plan_serves_many_calls() {
    let plan = Plan::new(16);
    for round in 0..5 {
        let values: Vec<f64> = (0..16).map(|i| (i + round) as f64).collect();
        let restored = plan.ifft(plan.fft(from_real(values.clone())));
        for (index, value) in values.iter().enumerate() {
            assert!(is_close(restored.real[index], *value), "{}", index);
        }
    }
}

#[test]
#[should_panic(expected = "Input must match the plan size.")]
fn rejects_input_of_another_size() {
    Plan::new(8).fft(from_real(vec![1.0, 2.0, 3.0, 4.0]));
}
