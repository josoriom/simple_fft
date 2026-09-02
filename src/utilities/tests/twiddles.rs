use crate::utilities::twiddles::get_twiddles;

#[test]
fn table_holds_half_the_size() {
    assert_eq!(get_twiddles(8).len(), 4);
    assert_eq!(get_twiddles(1024).len(), 512);
}

#[test]
fn first_factor_is_one() {
    let twiddles = get_twiddles(8);
    assert!((twiddles[0].real - 1.0).abs() < 1e-15);
    assert!(twiddles[0].imag.abs() < 1e-15);
}

#[test]
fn quarter_turn_is_minus_i() {
    let twiddles = get_twiddles(8);
    assert!(twiddles[2].real.abs() < 1e-15);
    assert!((twiddles[2].imag + 1.0).abs() < 1e-15);
}

#[test]
fn every_factor_sits_on_the_unit_circle() {
    for twiddle in get_twiddles(64) {
        assert!((twiddle.size() - 1.0).abs() < 1e-15);
    }
}
