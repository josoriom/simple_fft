use crate::structs::complex::Complex;

/// Argument accepted by the transforms.
/// `Real` is a real-valued sequence; `Complex` is a sequence already in the complex plane.
#[derive(Clone, Debug, PartialEq)]
pub enum Input {
    Real(Vec<f64>),
    Complex(Vec<Complex>),
}

impl Input {
    /// Length of the sequence, N.
    pub fn len(&self) -> usize {
        match self {
            Input::Real(values) => values.len(),
            Input::Complex(points) => points.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Embeds a real sequence in the complex plane as x + 0i.
    /// A sequence that is already complex is moved, not copied.
    pub fn into_points(self) -> Vec<Complex> {
        match self {
            Input::Real(values) => values
                .into_iter()
                .map(|value| Complex::new(value, 0.0))
                .collect(),
            Input::Complex(points) => points,
        }
    }
}

impl From<Vec<f64>> for Input {
    fn from(values: Vec<f64>) -> Self {
        Input::Real(values)
    }
}

impl From<Vec<Complex>> for Input {
    fn from(points: Vec<Complex>) -> Self {
        Input::Complex(points)
    }
}
