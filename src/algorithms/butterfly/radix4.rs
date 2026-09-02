use crate::Complex;

/// Multiplies by -i, a quarter turn: -i(a + bi) = b - ai.
#[inline]
fn quarter_turn(point: Complex) -> Complex {
    Complex::new(point.imag, -point.real)
}

/// Two radix-2 stages fused into one pass, so the block is read once instead of twice.
/// `inner` holds the twiddles of the first stage, `outer` those of the second,
/// which are the squares and the roots of each other.
pub(crate) fn apply(block: &mut [Complex], inner: &[Complex], outer: &[Complex]) {
    #[cfg(feature = "simd")]
    if crate::simd::apply_four(block, inner, outer) {
        return;
    }
    plain(block, inner, outer);
}

pub(crate) fn plain(block: &mut [Complex], inner: &[Complex], outer: &[Complex]) {
    let quarter = block.len() / 4;
    let (first, rest) = block.split_at_mut(quarter);
    let (second, rest) = rest.split_at_mut(quarter);
    let (third, fourth) = rest.split_at_mut(quarter);

    for index in 0..quarter {
        let turn = inner[index];
        let a = first[index];
        let b = turn * second[index];
        let c = third[index];
        let d = turn * fourth[index];

        let top = a + b;
        let bottom = a - b;
        let side = outer[index] * (c + d);
        let cross = quarter_turn(outer[index] * (c - d));

        first[index] = top + side;
        second[index] = bottom + cross;
        third[index] = top - side;
        fourth[index] = bottom - cross;
    }
}
