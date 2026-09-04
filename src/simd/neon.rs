use crate::utilities::twiddles::Turns;
use std::arch::aarch64::{
    float64x2_t, vaddq_f64, vfmaq_f64, vfmsq_f64, vld1q_f64, vmulq_f64, vnegq_f64, vst1q_f64,
    vsubq_f64,
};

pub(crate) const LANES: usize = 2;

#[inline]
unsafe fn multiply(
    turn_real: float64x2_t,
    turn_imag: float64x2_t,
    real: float64x2_t,
    imag: float64x2_t,
) -> (float64x2_t, float64x2_t) {
    (
        vfmsq_f64(vmulq_f64(turn_real, real), turn_imag, imag),
        vfmaq_f64(vmulq_f64(turn_real, imag), turn_imag, real),
    )
}

/// Two butterflies per step in 128 bit registers.
/// Caller must pass a block whose half is even and at least two long.
pub(crate) unsafe fn apply_two(real: &mut [f64], imag: &mut [f64], turns: Turns) {
    let half = real.len() / 2;
    let real_parts = real.as_mut_ptr();
    let imag_parts = imag.as_mut_ptr();
    let mut index = 0;
    while index < half {
        let far = index + half;
        let top_real = vld1q_f64(real_parts.add(index));
        let top_imag = vld1q_f64(imag_parts.add(index));
        let (bottom_real, bottom_imag) = multiply(
            vld1q_f64(turns.real.as_ptr().add(index)),
            vld1q_f64(turns.imag.as_ptr().add(index)),
            vld1q_f64(real_parts.add(far)),
            vld1q_f64(imag_parts.add(far)),
        );
        vst1q_f64(real_parts.add(index), vaddq_f64(top_real, bottom_real));
        vst1q_f64(imag_parts.add(index), vaddq_f64(top_imag, bottom_imag));
        vst1q_f64(real_parts.add(far), vsubq_f64(top_real, bottom_real));
        vst1q_f64(imag_parts.add(far), vsubq_f64(top_imag, bottom_imag));
        index += LANES;
    }
}

/// Two fused radix-4 butterflies per step, the wide form of `butterfly::radix4`.
/// Caller must pass a block whose quarter is even and at least two long.
pub(crate) unsafe fn apply_four(real: &mut [f64], imag: &mut [f64], inner: Turns, outer: Turns) {
    let quarter = real.len() / 4;
    let real_parts = real.as_mut_ptr();
    let imag_parts = imag.as_mut_ptr();
    let mut index = 0;
    while index < quarter {
        let second = index + quarter;
        let third = second + quarter;
        let fourth = third + quarter;
        let turn_real = vld1q_f64(inner.real.as_ptr().add(index));
        let turn_imag = vld1q_f64(inner.imag.as_ptr().add(index));

        let a_real = vld1q_f64(real_parts.add(index));
        let a_imag = vld1q_f64(imag_parts.add(index));
        let (b_real, b_imag) = multiply(
            turn_real,
            turn_imag,
            vld1q_f64(real_parts.add(second)),
            vld1q_f64(imag_parts.add(second)),
        );
        let c_real = vld1q_f64(real_parts.add(third));
        let c_imag = vld1q_f64(imag_parts.add(third));
        let (d_real, d_imag) = multiply(
            turn_real,
            turn_imag,
            vld1q_f64(real_parts.add(fourth)),
            vld1q_f64(imag_parts.add(fourth)),
        );

        let top_real = vaddq_f64(a_real, b_real);
        let top_imag = vaddq_f64(a_imag, b_imag);
        let bottom_real = vsubq_f64(a_real, b_real);
        let bottom_imag = vsubq_f64(a_imag, b_imag);

        let wide_real = vld1q_f64(outer.real.as_ptr().add(index));
        let wide_imag = vld1q_f64(outer.imag.as_ptr().add(index));
        let (side_real, side_imag) = multiply(
            wide_real,
            wide_imag,
            vaddq_f64(c_real, d_real),
            vaddq_f64(c_imag, d_imag),
        );
        let (turned_real, turned_imag) = multiply(
            wide_real,
            wide_imag,
            vsubq_f64(c_real, d_real),
            vsubq_f64(c_imag, d_imag),
        );
        let cross_real = turned_imag;
        let cross_imag = vnegq_f64(turned_real);

        vst1q_f64(real_parts.add(index), vaddq_f64(top_real, side_real));
        vst1q_f64(imag_parts.add(index), vaddq_f64(top_imag, side_imag));
        vst1q_f64(real_parts.add(second), vaddq_f64(bottom_real, cross_real));
        vst1q_f64(imag_parts.add(second), vaddq_f64(bottom_imag, cross_imag));
        vst1q_f64(real_parts.add(third), vsubq_f64(top_real, side_real));
        vst1q_f64(imag_parts.add(third), vsubq_f64(top_imag, side_imag));
        vst1q_f64(real_parts.add(fourth), vsubq_f64(bottom_real, cross_real));
        vst1q_f64(imag_parts.add(fourth), vsubq_f64(bottom_imag, cross_imag));
        index += LANES;
    }
}
