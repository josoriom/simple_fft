use super::{assert_same, sample};
use crate::algorithms::butterfly::scalar;
use crate::simd::avx2;
use crate::utilities::twiddles::{get_stage, get_twiddles};

#[test]
fn matches_the_plain_kernel() {
    if !avx2::is_available() {
        return;
    }
    let twiddles = get_twiddles(64);
    for half in [4usize, 8, 16, 32] {
        let stage = get_stage(&twiddles, half);
        let mut near_plain = sample(half, 0.3);
        let mut far_plain = sample(half, 7.1);
        let mut near_wide = near_plain.clone();
        let mut far_wide = far_plain.clone();

        scalar::apply(&mut near_plain, &mut far_plain, stage);
        unsafe { avx2::apply(&mut near_wide, &mut far_wide, stage) };

        assert_same(&near_plain, &near_wide, "near");
        assert_same(&far_plain, &far_wide, "far");
    }
}

#[test]
fn handles_four_points_at_a_time() {
    assert_eq!(avx2::LANES, 4);
}
