use crate::Complex;
use std::arch::x86_64::{
    __m256d, _mm256_add_pd, _mm256_fmadd_pd, _mm256_fmsub_pd, _mm256_loadu_pd, _mm256_mul_pd,
    _mm256_storeu_pd, _mm256_sub_pd, _mm256_unpackhi_pd, _mm256_unpacklo_pd,
};

/// Points handled per step by this kernel.
pub(crate) const LANES: usize = 4;

/// True when this processor can run the kernel. Checked once and cached by the standard library.
#[inline]
pub(crate) fn is_available() -> bool {
    is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma")
}

#[inline]
unsafe fn split_parts(parts: *const f64) -> (__m256d, __m256d) {
    let low = _mm256_loadu_pd(parts);
    let high = _mm256_loadu_pd(parts.add(4));
    (_mm256_unpacklo_pd(low, high), _mm256_unpackhi_pd(low, high))
}

#[inline]
unsafe fn join_parts(parts: *mut f64, real: __m256d, imag: __m256d) {
    _mm256_storeu_pd(parts, _mm256_unpacklo_pd(real, imag));
    _mm256_storeu_pd(parts.add(4), _mm256_unpackhi_pd(real, imag));
}

/// Four butterflies per step in 256 bit registers.
#[target_feature(enable = "avx2,fma")]
pub(crate) unsafe fn apply(near: &mut [Complex], far: &mut [Complex], twiddles: &[Complex]) {
    let count = near.len();
    let near_parts = near.as_mut_ptr() as *mut f64;
    let far_parts = far.as_mut_ptr() as *mut f64;
    let twiddle_parts = twiddles.as_ptr() as *const f64;
    let mut offset = 0;
    while offset < count {
        let place = offset * 2;
        let (top_real, top_imag) = split_parts(near_parts.add(place));
        let (point_real, point_imag) = split_parts(far_parts.add(place));
        let (twiddle_real, twiddle_imag) = split_parts(twiddle_parts.add(place));

        let bottom_real = _mm256_fmsub_pd(
            twiddle_real,
            point_real,
            _mm256_mul_pd(twiddle_imag, point_imag),
        );
        let bottom_imag = _mm256_fmadd_pd(
            twiddle_real,
            point_imag,
            _mm256_mul_pd(twiddle_imag, point_real),
        );

        join_parts(
            near_parts.add(place),
            _mm256_add_pd(top_real, bottom_real),
            _mm256_add_pd(top_imag, bottom_imag),
        );
        join_parts(
            far_parts.add(place),
            _mm256_sub_pd(top_real, bottom_real),
            _mm256_sub_pd(top_imag, bottom_imag),
        );
        offset += 4;
    }
}
