use crate::algorithms::butterfly::{apply, scalar};
use crate::utilities::twiddles::{get_stage, get_twiddles};
use crate::Complex;

#[test]
fn a_single_pair_adds_and_subtracts() {
    let mut near = vec![Complex::new(5.0, 1.0)];
    let mut far = vec![Complex::new(2.0, 3.0)];
    scalar::apply(&mut near, &mut far, &[Complex::new(1.0, 0.0)]);
    assert_eq!(near[0], Complex::new(7.0, 4.0));
    assert_eq!(far[0], Complex::new(3.0, -2.0));
}

#[test]
fn a_twiddle_of_minus_i_turns_the_point() {
    let mut near = vec![Complex::new(0.0, 0.0)];
    let mut far = vec![Complex::new(1.0, 0.0)];
    scalar::apply(&mut near, &mut far, &[Complex::new(0.0, -1.0)]);
    assert_eq!(near[0], Complex::new(0.0, -1.0));
    assert_eq!(far[0], Complex::new(0.0, 1.0));
}

#[test]
fn the_chosen_kernel_agrees_with_the_plain_one() {
    let twiddles = get_twiddles(64);
    for half in [1usize, 2, 4, 8, 16, 32] {
        let stage = get_stage(&twiddles, half);
        let points: Vec<Complex> = (0..half * 2)
            .map(|index| Complex::new((index as f64).sin(), (index as f64).cos()))
            .collect();

        let mut plain = points.clone();
        let (near_plain, far_plain) = plain.split_at_mut(half);
        scalar::apply(near_plain, far_plain, stage);

        let mut chosen = points.clone();
        let (near_chosen, far_chosen) = chosen.split_at_mut(half);
        apply(near_chosen, far_chosen, stage);

        for index in 0..half * 2 {
            assert!(
                (plain[index].real - chosen[index].real).abs() < 1e-15,
                "half {}",
                half
            );
            assert!(
                (plain[index].imag - chosen[index].imag).abs() < 1e-15,
                "half {}",
                half
            );
        }
    }
}
