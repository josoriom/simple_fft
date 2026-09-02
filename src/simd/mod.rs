#[cfg(target_arch = "x86_64")]
pub mod avx2;
#[cfg(target_arch = "aarch64")]
pub mod neon;

#[cfg(test)]
mod tests;

use crate::Complex;

/// Runs the widest butterfly kernel this target offers.
/// Answers false when the block is too short to fill a register, or the processor has no
/// wide kernel at all, so the caller falls back to the plain one.
#[inline]
pub(crate) fn apply(near: &mut [Complex], far: &mut [Complex], twiddles: &[Complex]) -> bool {
    #[cfg(target_arch = "aarch64")]
    if near.len() >= neon::LANES {
        unsafe { neon::apply(near, far, twiddles) };
        return true;
    }
    #[cfg(target_arch = "x86_64")]
    if near.len() >= avx2::LANES && avx2::is_available() {
        unsafe { avx2::apply(near, far, twiddles) };
        return true;
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    let _ = (near, far, twiddles);
    false
}

/// Runs the widest fused radix-4 kernel this target offers.
/// Answers false when none fits, so the caller falls back to the plain one.
#[inline]
pub(crate) fn apply_four(block: &mut [Complex], inner: &[Complex], outer: &[Complex]) -> bool {
    #[cfg(target_arch = "aarch64")]
    if block.len() / 4 >= neon::LANES {
        unsafe { neon::apply_four(block, inner, outer) };
        return true;
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = (block, inner, outer);
    false
}
