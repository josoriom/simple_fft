use crate::utilities::twiddles::Turns;
use crate::Complex;

/// Multiplies by -i, a quarter turn: -i(a + bi) = b - ai.
#[inline]
fn quarter_turn(point: Complex) -> Complex {
    Complex::new(point.imag, -point.real)
}

/// Two radix-2 stages fused into one pass, so the block is read once instead of twice.
/// `inner` holds the twiddles of the first stage, `outer` those of the second,
/// which are the squares and the roots of each other.
pub(crate) fn apply(real: &mut [f64], imag: &mut [f64], inner: Turns, outer: Turns) {
    #[cfg(feature = "simd")]
    if crate::simd::apply_four(real, imag, inner, outer) {
        return;
    }
    plain(real, imag, inner, outer);
}

pub(crate) fn plain(real: &mut [f64], imag: &mut [f64], inner: Turns, outer: Turns) {
    let quarter = real.len() / 4;
    for index in 0..quarter {
        let second = index + quarter;
        let third = second + quarter;
        let fourth = third + quarter;
        let turn = Complex::new(inner.real[index], inner.imag[index]);

        let a = Complex::new(real[index], imag[index]);
        let b = turn * Complex::new(real[second], imag[second]);
        let c = Complex::new(real[third], imag[third]);
        let d = turn * Complex::new(real[fourth], imag[fourth]);

        let top = a + b;
        let bottom = a - b;
        let wide = Complex::new(outer.real[index], outer.imag[index]);
        let side = wide * (c + d);
        let cross = quarter_turn(wide * (c - d));

        write(real, imag, index, top + side);
        write(real, imag, second, bottom + cross);
        write(real, imag, third, top - side);
        write(real, imag, fourth, bottom - cross);
    }
}

#[inline]
fn write(real: &mut [f64], imag: &mut [f64], index: usize, point: Complex) {
    real[index] = point.real;
    imag[index] = point.imag;
}
