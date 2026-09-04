use crate::utilities::twiddles::Turns;
use std::arch::x86_64::{
    __m256d, _mm256_add_pd, _mm256_fmadd_pd, _mm256_fmsub_pd, _mm256_loadu_pd, _mm256_mul_pd,
    _mm256_setzero_pd, _mm256_storeu_pd, _mm256_sub_pd,
};

pub(crate) const LANES: usize = 4;

/// True when this processor can run the kernel. Checked once and cached by the standard library.
#[inline]
pub(crate) fn is_available() -> bool {
    is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma")
}

#[inline]
unsafe fn multiply(
    turn_real: __m256d,
    turn_imag: __m256d,
    real: __m256d,
    imag: __m256d,
) -> (__m256d, __m256d) {
    (
        _mm256_fmsub_pd(turn_real, real, _mm256_mul_pd(turn_imag, imag)),
        _mm256_fmadd_pd(turn_real, imag, _mm256_mul_pd(turn_imag, real)),
    )
}

/// Four butterflies per step in 256 bit registers.
/// Caller must pass a block whose half is a multiple of four.
#[target_feature(enable = "avx2,fma")]
pub(crate) unsafe fn apply_two(real: &mut [f64], imag: &mut [f64], turns: Turns) {
    let half = real.len() / 2;
    let real_parts = real.as_mut_ptr();
    let imag_parts = imag.as_mut_ptr();
    let mut index = 0;
    while index < half {
        let far = index + half;
        let top_real = _mm256_loadu_pd(real_parts.add(index));
        let top_imag = _mm256_loadu_pd(imag_parts.add(index));
        let (bottom_real, bottom_imag) = multiply(
            _mm256_loadu_pd(turns.real.as_ptr().add(index)),
            _mm256_loadu_pd(turns.imag.as_ptr().add(index)),
            _mm256_loadu_pd(real_parts.add(far)),
            _mm256_loadu_pd(imag_parts.add(far)),
        );
        _mm256_storeu_pd(real_parts.add(index), _mm256_add_pd(top_real, bottom_real));
        _mm256_storeu_pd(imag_parts.add(index), _mm256_add_pd(top_imag, bottom_imag));
        _mm256_storeu_pd(real_parts.add(far), _mm256_sub_pd(top_real, bottom_real));
        _mm256_storeu_pd(imag_parts.add(far), _mm256_sub_pd(top_imag, bottom_imag));
        index += LANES;
    }
}

/// Four fused radix-4 butterflies per step, the wide form of `butterfly::radix4`.
/// Caller must pass a block whose quarter is a multiple of four.
#[target_feature(enable = "avx2,fma")]
pub(crate) unsafe fn apply_four(real: &mut [f64], imag: &mut [f64], inner: Turns, outer: Turns) {
    let quarter = real.len() / 4;
    let real_parts = real.as_mut_ptr();
    let imag_parts = imag.as_mut_ptr();
    let mut index = 0;
    while index < quarter {
        let second = index + quarter;
        let third = second + quarter;
        let fourth = third + quarter;
        let turn_real = _mm256_loadu_pd(inner.real.as_ptr().add(index));
        let turn_imag = _mm256_loadu_pd(inner.imag.as_ptr().add(index));

        let a_real = _mm256_loadu_pd(real_parts.add(index));
        let a_imag = _mm256_loadu_pd(imag_parts.add(index));
        let (b_real, b_imag) = multiply(
            turn_real,
            turn_imag,
            _mm256_loadu_pd(real_parts.add(second)),
            _mm256_loadu_pd(imag_parts.add(second)),
        );
        let c_real = _mm256_loadu_pd(real_parts.add(third));
        let c_imag = _mm256_loadu_pd(imag_parts.add(third));
        let (d_real, d_imag) = multiply(
            turn_real,
            turn_imag,
            _mm256_loadu_pd(real_parts.add(fourth)),
            _mm256_loadu_pd(imag_parts.add(fourth)),
        );

        let top_real = _mm256_add_pd(a_real, b_real);
        let top_imag = _mm256_add_pd(a_imag, b_imag);
        let bottom_real = _mm256_sub_pd(a_real, b_real);
        let bottom_imag = _mm256_sub_pd(a_imag, b_imag);

        let wide_real = _mm256_loadu_pd(outer.real.as_ptr().add(index));
        let wide_imag = _mm256_loadu_pd(outer.imag.as_ptr().add(index));
        let (side_real, side_imag) = multiply(
            wide_real,
            wide_imag,
            _mm256_add_pd(c_real, d_real),
            _mm256_add_pd(c_imag, d_imag),
        );
        let (turned_real, turned_imag) = multiply(
            wide_real,
            wide_imag,
            _mm256_sub_pd(c_real, d_real),
            _mm256_sub_pd(c_imag, d_imag),
        );
        let cross_real = turned_imag;
        let cross_imag = _mm256_sub_pd(_mm256_setzero_pd(), turned_real);

        _mm256_storeu_pd(real_parts.add(index), _mm256_add_pd(top_real, side_real));
        _mm256_storeu_pd(imag_parts.add(index), _mm256_add_pd(top_imag, side_imag));
        _mm256_storeu_pd(
            real_parts.add(second),
            _mm256_add_pd(bottom_real, cross_real),
        );
        _mm256_storeu_pd(
            imag_parts.add(second),
            _mm256_add_pd(bottom_imag, cross_imag),
        );
        _mm256_storeu_pd(real_parts.add(third), _mm256_sub_pd(top_real, side_real));
        _mm256_storeu_pd(imag_parts.add(third), _mm256_sub_pd(top_imag, side_imag));
        _mm256_storeu_pd(
            real_parts.add(fourth),
            _mm256_sub_pd(bottom_real, cross_real),
        );
        _mm256_storeu_pd(
            imag_parts.add(fourth),
            _mm256_sub_pd(bottom_imag, cross_imag),
        );
        index += LANES;
    }
}
