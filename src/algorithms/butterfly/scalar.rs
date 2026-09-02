use crate::Complex;

/// One butterfly per step, the plain form.
/// near[i] = near[i] + twiddles[i] * far[i], far[i] = near[i] - twiddles[i] * far[i].
pub(crate) fn apply(near: &mut [Complex], far: &mut [Complex], twiddles: &[Complex]) {
    for offset in 0..near.len() {
        let top = near[offset];
        let bottom = twiddles[offset] * far[offset];
        near[offset] = top + bottom;
        far[offset] = top - bottom;
    }
}
