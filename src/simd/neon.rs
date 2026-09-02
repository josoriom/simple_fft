use crate::Complex;
use std::arch::aarch64::{
    float64x2_t, float64x2x2_t, vaddq_f64, vfmaq_f64, vfmsq_f64, vld2q_f64, vmulq_f64,
    vnegq_f64, vst2q_f64, vsubq_f64,
};

/// Points handled per step by this kernel.
pub(crate) const LANES: usize = 2;

/// Two butterflies per step in 128 bit registers.
/// `vld2q_f64` splits the interleaved points into one register of real parts and one of imaginary.
/// Caller must pass three slices of the same even length, at least two long, that do not overlap.
pub(crate) unsafe fn apply(near: &mut [Complex], far: &mut [Complex], twiddles: &[Complex]) {
    let count = near.len();
    let near_parts = near.as_mut_ptr() as *mut f64;
    let far_parts = far.as_mut_ptr() as *mut f64;
    let twiddle_parts = twiddles.as_ptr() as *const f64;
    let mut offset = 0;
    while offset < count {
        let place = offset * 2;
        let top = vld2q_f64(near_parts.add(place));
        let point = vld2q_f64(far_parts.add(place));
        let twiddle = vld2q_f64(twiddle_parts.add(place));

        let bottom_real = vfmsq_f64(vmulq_f64(twiddle.0, point.0), twiddle.1, point.1);
        let bottom_imag = vfmaq_f64(vmulq_f64(twiddle.0, point.1), twiddle.1, point.0);

        vst2q_f64(
            near_parts.add(place),
            float64x2x2_t(vaddq_f64(top.0, bottom_real), vaddq_f64(top.1, bottom_imag)),
        );
        vst2q_f64(
            far_parts.add(place),
            float64x2x2_t(vsubq_f64(top.0, bottom_real), vsubq_f64(top.1, bottom_imag)),
        );
        offset += 2;
    }
}

#[inline]
unsafe fn multiply(
    left: float64x2x2_t,
    right_real: float64x2_t,
    right_imag: float64x2_t,
) -> (float64x2_t, float64x2_t) {
    (
        vfmsq_f64(vmulq_f64(left.0, right_real), left.1, right_imag),
        vfmaq_f64(vmulq_f64(left.0, right_imag), left.1, right_real),
    )
}

/// Two fused radix-4 butterflies per step, the wide form of `butterfly::radix4`.
/// Caller must pass a block whose quarter is even and at least two long.
pub(crate) unsafe fn apply_four(block: &mut [Complex], inner: &[Complex], outer: &[Complex]) {
    let quarter = block.len() / 4;
    let base = block.as_mut_ptr() as *mut f64;
    let first = base;
    let second = base.add(quarter * 2);
    let third = base.add(quarter * 4);
    let fourth = base.add(quarter * 6);
    let inner_parts = inner.as_ptr() as *const f64;
    let outer_parts = outer.as_ptr() as *const f64;

    let mut offset = 0;
    while offset < quarter {
        let place = offset * 2;
        let a = vld2q_f64(first.add(place));
        let c = vld2q_f64(third.add(place));
        let inner_turn = vld2q_f64(inner_parts.add(place));
        let outer_turn = vld2q_f64(outer_parts.add(place));

        let second_point = vld2q_f64(second.add(place));
        let fourth_point = vld2q_f64(fourth.add(place));
        let (b_real, b_imag) = multiply(inner_turn, second_point.0, second_point.1);
        let (d_real, d_imag) = multiply(inner_turn, fourth_point.0, fourth_point.1);

        let top_real = vaddq_f64(a.0, b_real);
        let top_imag = vaddq_f64(a.1, b_imag);
        let bottom_real = vsubq_f64(a.0, b_real);
        let bottom_imag = vsubq_f64(a.1, b_imag);

        let (side_real, side_imag) = multiply(
            outer_turn,
            vaddq_f64(c.0, d_real),
            vaddq_f64(c.1, d_imag),
        );
        let (turned_real, turned_imag) = multiply(
            outer_turn,
            vsubq_f64(c.0, d_real),
            vsubq_f64(c.1, d_imag),
        );
        let cross_real = turned_imag;
        let cross_imag = vnegq_f64(turned_real);

        vst2q_f64(
            first.add(place),
            float64x2x2_t(vaddq_f64(top_real, side_real), vaddq_f64(top_imag, side_imag)),
        );
        vst2q_f64(
            second.add(place),
            float64x2x2_t(vaddq_f64(bottom_real, cross_real), vaddq_f64(bottom_imag, cross_imag)),
        );
        vst2q_f64(
            third.add(place),
            float64x2x2_t(vsubq_f64(top_real, side_real), vsubq_f64(top_imag, side_imag)),
        );
        vst2q_f64(
            fourth.add(place),
            float64x2x2_t(vsubq_f64(bottom_real, cross_real), vsubq_f64(bottom_imag, cross_imag)),
        );
        offset += LANES;
    }
}
