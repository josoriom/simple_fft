use std::ops::{Add, Div, Mul, Sub};

/// Complex number in rectangular form, z = a + bi.
/// `real` is a, `imag` is b, and i is the imaginary unit where i^2 = -1.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    #[inline]
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// Additive identity, z = 0 + 0i.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Euler's formula: e^(i*angle) = cos(angle) + i*sin(angle).
    /// Gives the point on the unit circle at `angle` radians.
    #[inline]
    pub fn from_angle(angle: f64) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    /// Complex conjugate: conj(a + bi) = a - bi.
    #[inline]
    pub fn flip_sign_of_imag(self) -> Self {
        Self::new(self.real, -self.imag)
    }

    /// Scalar multiplication: r(a + bi) = ra + rbi.
    #[inline]
    pub fn scale(self, factor: f64) -> Self {
        Self::new(self.real * factor, self.imag * factor)
    }

    /// Modulus: |a + bi| = sqrt(a^2 + b^2).
    #[inline]
    pub fn size(self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

/// Addition is componentwise: (a + bi) + (c + di) = (a + c) + (b + d)i.
impl Add for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

/// Subtraction is componentwise: (a + bi) - (c + di) = (a - c) + (b - d)i.
impl Sub for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

/// Distributive expansion with i^2 = -1 substituted:
/// (a + bi)(c + di) = (ac - bd) + (ad + bc)i.
impl Mul for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

/// Multiply by the conjugate of the divisor, then divide by its squared modulus:
/// (a + bi) / (c + di) = ((ac + bd) + (bc - ad)i) / (c^2 + d^2).
impl Div for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let divisor = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / divisor,
            (self.imag * other.real - self.real * other.imag) / divisor,
        )
    }
}
