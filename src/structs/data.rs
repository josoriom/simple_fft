/// A signal held as two arrays, the real parts and the imaginary parts.
/// Both arrays must have the same length.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Data {
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl Data {
    pub fn len(&self) -> usize {
        self.real.len()
    }

    pub fn is_empty(&self) -> bool {
        self.real.is_empty()
    }
}
