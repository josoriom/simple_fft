#[cfg(target_arch = "x86_64")]
mod avx2;
mod dispatch;
#[cfg(target_arch = "aarch64")]
mod neon;

/// Repeatable parts for comparing a wide kernel against the plain one.
pub(super) fn sample(count: usize, seed: f64) -> (Vec<f64>, Vec<f64>) {
    let real = (0..count)
        .map(|i| ((i as f64 + seed).sin()) * 3.0)
        .collect();
    let imag = (0..count).map(|i| (i as f64 + seed).cos() - 0.5).collect();
    (real, imag)
}

/// Fails unless every part agrees to a few of the last bits of a double.
/// A wide kernel fuses multiply and add, which rounds once where the plain one rounds twice,
/// so the two disagree by well under one unit in the last place.
pub(super) fn assert_same(left: &[f64], right: &[f64], label: &str) {
    for index in 0..left.len() {
        let room = 1e-14 * (1.0 + left[index].abs());
        assert!(
            (left[index] - right[index]).abs() < room,
            "{} at {}",
            label,
            index
        );
    }
}
