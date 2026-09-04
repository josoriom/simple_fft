use crate::utilities::twiddles::Twiddles;

#[test]
fn table_holds_one_less_than_the_size() {
    assert_eq!(Twiddles::new(8).len(), 7);
    assert_eq!(Twiddles::new(1024).len(), 1023);
}

#[test]
fn short_sizes_need_no_factors() {
    assert_eq!(Twiddles::new(1).len(), 0);
}

#[test]
fn every_stage_starts_at_one() {
    let twiddles = Twiddles::new(16);
    for half in [1usize, 2, 4, 8] {
        let stage = twiddles.get_stage(half);
        assert_eq!(stage.real.len(), half);
        assert!((stage.real[0] - 1.0).abs() < 1e-15);
        assert!(stage.imag[0].abs() < 1e-15);
    }
}

#[test]
fn quarter_turn_is_minus_i() {
    let twiddles = Twiddles::new(8);
    let stage = twiddles.get_stage(2);
    assert!(stage.real[1].abs() < 1e-15);
    assert!((stage.imag[1] + 1.0).abs() < 1e-15);
}

#[test]
fn every_factor_sits_on_the_unit_circle() {
    let twiddles = Twiddles::new(64);
    let stage = twiddles.get_stage(32);
    for index in 0..stage.real.len() {
        let size = stage.real[index] * stage.real[index] + stage.imag[index] * stage.imag[index];
        assert!((size - 1.0).abs() < 1e-15);
    }
}
