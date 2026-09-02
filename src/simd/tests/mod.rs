#[cfg(target_arch = "x86_64")]
mod avx2;
#[cfg(target_arch = "aarch64")]
mod neon;
mod dispatch;

use crate::Complex;

/// Repeatable points for comparing a wide kernel against the plain one.
pub(super) fn sample(count: usize, seed: f64) -> Vec<Complex> {
    (0..count)
        .map(|index| {
            let step = index as f64 + seed;
            Complex::new(step.sin() * 3.0, step.cos() - 0.5)
        })
        .collect()
}

/// Fails unless every point agrees to a few of the last bits of a double.
/// A wide kernel fuses multiply and add, which rounds once where the plain one rounds twice,
/// so the two disagree by well under one unit in the last place.
pub(super) fn assert_same(left: &[Complex], right: &[Complex], label: &str) {
    for index in 0..left.len() {
        let point = left[index];
        let other = right[index];
        let room = 1e-14 * (1.0 + point.real.abs().max(point.imag.abs()));
        assert!((point.real - other.real).abs() < room, "{} real at {}", label, index);
        assert!((point.imag - other.imag).abs() < room, "{} imag at {}", label, index);
    }
}
