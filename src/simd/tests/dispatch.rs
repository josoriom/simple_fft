use super::sample;
use crate::simd::apply_two;
use crate::utilities::twiddles::Twiddles;

#[test]
fn refuses_a_block_too_short_for_a_register() {
    let twiddles = Twiddles::new(8);
    let turns = twiddles.get_stage(1);
    let (mut real, mut imag) = sample(2, 0.3);
    assert!(!apply_two(&mut real, &mut imag, turns));
}

#[test]
fn takes_a_block_that_fills_a_register() {
    let twiddles = Twiddles::new(64);
    let turns = twiddles.get_stage(32);
    let (mut real, mut imag) = sample(64, 0.3);
    let taken = apply_two(&mut real, &mut imag, turns);
    assert_eq!(
        taken,
        cfg!(any(target_arch = "aarch64", target_arch = "x86_64"))
    );
}
