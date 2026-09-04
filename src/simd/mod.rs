#[cfg(target_arch = "x86_64")]
pub mod avx2;
#[cfg(target_arch = "aarch64")]
pub mod neon;

#[cfg(test)]
mod tests;

use crate::utilities::twiddles::Turns;

/// Runs the widest butterfly kernel this target offers.
/// Answers false when none fits, so the caller falls back to the plain one.
#[inline]
pub(crate) fn apply_two(real: &mut [f64], imag: &mut [f64], turns: Turns) -> bool {
    #[cfg(target_arch = "aarch64")]
    if real.len() / 2 >= neon::LANES {
        unsafe { neon::apply_two(real, imag, turns) };
        return true;
    }
    #[cfg(target_arch = "x86_64")]
    if real.len() / 2 >= avx2::LANES && avx2::is_available() {
        unsafe { avx2::apply_two(real, imag, turns) };
        return true;
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    let _ = (real, imag, turns);
    false
}

/// Runs the widest fused radix-4 kernel this target offers.
/// Answers false when none fits, so the caller falls back to the plain one.
#[inline]
pub(crate) fn apply_four(real: &mut [f64], imag: &mut [f64], inner: Turns, outer: Turns) -> bool {
    #[cfg(target_arch = "aarch64")]
    if real.len() / 4 >= neon::LANES {
        unsafe { neon::apply_four(real, imag, inner, outer) };
        return true;
    }
    #[cfg(target_arch = "x86_64")]
    if real.len() / 4 >= avx2::LANES && avx2::is_available() {
        unsafe { avx2::apply_four(real, imag, inner, outer) };
        return true;
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    let _ = (real, imag, inner, outer);
    false
}
