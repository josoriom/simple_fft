use crate::algorithms::cooley_tukey::cooley_tukey;
use crate::utilities::twiddles::get_twiddles;
use crate::{Complex, Input};

/// Holds the twiddle table for one size, so repeated transforms build it only once.
pub struct Plan {
    size: usize,
    twiddles: Vec<Complex>,
}

impl Plan {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            twiddles: get_twiddles(size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn fft(&self, input: Input) -> Vec<Complex> {
        let mut data = self.take(input);
        cooley_tukey(&mut data, &self.twiddles);
        data
    }

    pub fn ifft(&self, input: Input) -> Vec<Complex> {
        let mut data = self.take(input);
        for point in data.iter_mut() {
            *point = point.flip_sign_of_imag();
        }
        cooley_tukey(&mut data, &self.twiddles);
        let factor = 1.0 / self.size as f64;
        for point in data.iter_mut() {
            *point = point.flip_sign_of_imag().scale(factor);
        }
        data
    }

    fn take(&self, input: Input) -> Vec<Complex> {
        assert_eq!(input.len(), self.size, "Input must match the plan size.");
        input.into_points()
    }
}
