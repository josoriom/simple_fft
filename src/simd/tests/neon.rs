use super::{assert_same, sample};
use crate::algorithms::butterfly::{radix4, scalar};
use crate::simd::neon;
use crate::utilities::twiddles::Twiddles;

#[test]
fn matches_the_plain_kernel() {
    let twiddles = Twiddles::new(256);
    for half in [2usize, 4, 8, 16, 32] {
        let turns = twiddles.get_stage(half);
        let (mut plain_real, mut plain_imag) = sample(half * 2, 0.3);
        let mut wide_real = plain_real.clone();
        let mut wide_imag = plain_imag.clone();

        scalar::apply(&mut plain_real, &mut plain_imag, turns);
        unsafe { neon::apply_two(&mut wide_real, &mut wide_imag, turns) };

        assert_same(&plain_real, &wide_real, "real");
        assert_same(&plain_imag, &wide_imag, "imag");
    }
}

#[test]
fn radix_four_matches_the_plain_kernel() {
    let twiddles = Twiddles::new(256);
    for quarter in [2usize, 4, 8, 16, 32] {
        let inner = twiddles.get_stage(quarter);
        let outer = twiddles.get_stage(quarter * 2);
        let (mut plain_real, mut plain_imag) = sample(quarter * 4, 0.9);
        let mut wide_real = plain_real.clone();
        let mut wide_imag = plain_imag.clone();

        radix4::plain(&mut plain_real, &mut plain_imag, inner, outer);
        unsafe { neon::apply_four(&mut wide_real, &mut wide_imag, inner, outer) };

        assert_same(&plain_real, &wide_real, "radix four real");
        assert_same(&plain_imag, &wide_imag, "radix four imag");
    }
}

#[test]
fn handles_two_points_at_a_time() {
    assert_eq!(neon::LANES, 2);
}
