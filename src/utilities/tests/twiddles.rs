use crate::utilities::twiddles::{get_stage, get_twiddles};

#[test]
fn table_holds_one_less_than_the_size() {
    assert_eq!(get_twiddles(8).len(), 7);
    assert_eq!(get_twiddles(1024).len(), 1023);
}

#[test]
fn short_sizes_need_no_factors() {
    assert!(get_twiddles(1).is_empty());
}

#[test]
fn every_stage_starts_at_one() {
    let twiddles = get_twiddles(16);
    for half in [1usize, 2, 4, 8] {
        let stage = get_stage(&twiddles, half);
        assert_eq!(stage.len(), half);
        assert!((stage[0].real - 1.0).abs() < 1e-15);
        assert!(stage[0].imag.abs() < 1e-15);
    }
}

#[test]
fn quarter_turn_is_minus_i() {
    let twiddles = get_twiddles(8);
    let stage = get_stage(&twiddles, 2);
    assert!(stage[1].real.abs() < 1e-15);
    assert!((stage[1].imag + 1.0).abs() < 1e-15);
}

#[test]
fn every_factor_sits_on_the_unit_circle() {
    for twiddle in get_twiddles(64) {
        assert!((twiddle.size() - 1.0).abs() < 1e-15);
    }
}
