use std::f64::consts::TAU;

/// Twiddle factors for every stage, kept as two arrays and laid out back to back
/// so each stage reads a contiguous slice. Total length is size - 1.
pub(crate) struct Twiddles {
    real: Vec<f64>,
    imag: Vec<f64>,
}

/// The factors one stage reads.
#[derive(Copy, Clone)]
pub(crate) struct Turns<'a> {
    pub real: &'a [f64],
    pub imag: &'a [f64],
}

impl Twiddles {
    /// Stage with span 2^s holds W = e^(-2*pi*i*k/span) for k in 0..span/2.
    pub(crate) fn new(size: usize) -> Self {
        let mut real = Vec::with_capacity(size.saturating_sub(1));
        let mut imag = Vec::with_capacity(size.saturating_sub(1));
        let mut span = 2;
        while span <= size {
            for index in 0..span / 2 {
                let angle = -TAU * index as f64 / span as f64;
                real.push(angle.cos());
                imag.push(angle.sin());
            }
            span *= 2;
        }
        Self { real, imag }
    }

    /// Factors belonging to the stage whose half span is `half`.
    #[inline]
    pub(crate) fn get_stage(&self, half: usize) -> Turns<'_> {
        Turns {
            real: &self.real[half - 1..half * 2 - 1],
            imag: &self.imag[half - 1..half * 2 - 1],
        }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.real.len()
    }
}
