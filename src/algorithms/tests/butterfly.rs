use crate::algorithms::butterfly::{apply, scalar};
use crate::utilities::twiddles::Twiddles;

#[test]
fn a_single_pair_adds_and_subtracts() {
    let mut real = vec![5.0, 2.0];
    let mut imag = vec![1.0, 3.0];
    let twiddles = Twiddles::new(2);
    scalar::apply(&mut real, &mut imag, twiddles.get_stage(1));
    assert_eq!(real, vec![7.0, 3.0]);
    assert_eq!(imag, vec![4.0, -2.0]);
}

#[test]
fn the_chosen_kernel_agrees_with_the_plain_one() {
    let twiddles = Twiddles::new(64);
    for half in [1usize, 2, 4, 8, 16, 32] {
        let turns = twiddles.get_stage(half);
        let real: Vec<f64> = (0..half * 2).map(|i| (i as f64).sin()).collect();
        let imag: Vec<f64> = (0..half * 2).map(|i| (i as f64).cos()).collect();

        let mut plain_real = real.clone();
        let mut plain_imag = imag.clone();
        scalar::apply(&mut plain_real, &mut plain_imag, turns);

        let mut chosen_real = real.clone();
        let mut chosen_imag = imag.clone();
        apply(&mut chosen_real, &mut chosen_imag, turns);

        for index in 0..half * 2 {
            assert!(
                (plain_real[index] - chosen_real[index]).abs() < 1e-14,
                "half {}",
                half
            );
            assert!(
                (plain_imag[index] - chosen_imag[index]).abs() < 1e-14,
                "half {}",
                half
            );
        }
    }
}
