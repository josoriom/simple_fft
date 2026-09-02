use super::{assert_same, sample};
use crate::algorithms::butterfly::scalar;
use crate::simd::neon;
use crate::utilities::twiddles::{get_stage, get_twiddles};

#[test]
fn matches_the_plain_kernel() {
    let twiddles = get_twiddles(64);
    for half in [2usize, 4, 8, 16, 32] {
        let stage = get_stage(&twiddles, half);
        let mut near_plain = sample(half, 0.3);
        let mut far_plain = sample(half, 7.1);
        let mut near_wide = near_plain.clone();
        let mut far_wide = far_plain.clone();

        scalar::apply(&mut near_plain, &mut far_plain, stage);
        unsafe { neon::apply(&mut near_wide, &mut far_wide, stage) };

        assert_same(&near_plain, &near_wide, "near");
        assert_same(&far_plain, &far_wide, "far");
    }
}

#[test]
fn handles_two_points_at_a_time() {
    assert_eq!(neon::LANES, 2);
}

#[test]
fn radix_four_matches_the_plain_kernel() {
    use crate::algorithms::butterfly::radix4;
    let twiddles = get_twiddles(256);
    for quarter in [2usize, 4, 8, 16, 32] {
        let inner = get_stage(&twiddles, quarter);
        let outer = get_stage(&twiddles, quarter * 2);
        let mut plain = sample(quarter * 4, 0.9);
        let mut wide = plain.clone();

        radix4::plain(&mut plain, inner, outer);
        unsafe { neon::apply_four(&mut wide, inner, outer) };

        assert_same(&plain, &wide, "radix four");
    }
}
