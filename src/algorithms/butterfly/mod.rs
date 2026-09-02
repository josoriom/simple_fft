pub mod radix4;
pub mod scalar;

use crate::Complex;

/// Runs one stage of butterflies over a block.
/// Hands the block to a wide kernel when the `simd` feature is on and one fits,
/// and uses the plain kernel otherwise.
#[inline]
pub(crate) fn apply(near: &mut [Complex], far: &mut [Complex], twiddles: &[Complex]) {
    #[cfg(feature = "simd")]
    if crate::simd::apply(near, far, twiddles) {
        return;
    }
    scalar::apply(near, far, twiddles);
}
