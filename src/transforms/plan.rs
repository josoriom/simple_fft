use crate::algorithms::cooley_tukey::cooley_tukey;
use crate::utilities::twiddles::Twiddles;
use crate::Data;

/// Holds the twiddle table for one size, so repeated transforms build it only once.
pub struct Plan {
    size: usize,
    twiddles: Twiddles,
}

impl Plan {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            twiddles: Twiddles::new(size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn fft(&self, input: Data) -> Data {
        let mut data = self.take(input);
        cooley_tukey(&mut data.real, &mut data.imag, &self.twiddles);
        data
    }

    pub fn ifft(&self, input: Data) -> Data {
        let mut data = self.take(input);
        for part in data.imag.iter_mut() {
            *part = -*part;
        }
        cooley_tukey(&mut data.real, &mut data.imag, &self.twiddles);
        let factor = 1.0 / self.size as f64;
        for part in data.real.iter_mut() {
            *part *= factor;
        }
        for part in data.imag.iter_mut() {
            *part *= -factor;
        }
        data
    }

    fn take(&self, input: Data) -> Data {
        assert_eq!(
            input.real.len(),
            input.imag.len(),
            "Real and imaginary parts must have the same length."
        );
        assert_eq!(input.len(), self.size, "Input must match the plan size.");
        input
    }
}
