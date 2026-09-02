use super::sample;
use crate::simd::apply;
use crate::utilities::twiddles::{get_stage, get_twiddles};

#[test]
fn refuses_a_block_too_short_for_a_register() {
    let twiddles = get_twiddles(8);
    let stage = get_stage(&twiddles, 1);
    let mut near = sample(1, 0.3);
    let mut far = sample(1, 7.1);
    assert!(!apply(&mut near, &mut far, stage));
}

#[test]
fn takes_a_block_that_fills_a_register() {
    let twiddles = get_twiddles(64);
    let stage = get_stage(&twiddles, 32);
    let mut near = sample(32, 0.3);
    let mut far = sample(32, 7.1);
    let taken = apply(&mut near, &mut far, stage);
    assert_eq!(
        taken,
        cfg!(any(target_arch = "aarch64", target_arch = "x86_64"))
    );
}
