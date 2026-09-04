pub mod radix4;
pub mod scalar;

use crate::utilities::twiddles::Turns;

/// Runs one stage of butterflies, on a wide kernel when the `simd` feature offers one.
#[inline]
pub(crate) fn apply(real: &mut [f64], imag: &mut [f64], turns: Turns) {
    #[cfg(feature = "simd")]
    if crate::simd::apply_two(real, imag, turns) {
        return;
    }
    scalar::apply(real, imag, turns);
}
