use rustfft::{Fft, FftPlanner, num_complex::Complex};
use std::sync::Arc;

/// A structure that plans both the forward and inverse transforms
/// for data of a specified length, allowing efficient FFT computation.
#[derive(Clone)]
pub struct FFT {
    forward_fft: Arc<dyn Fft<f64>>,
    inverse_fft: Arc<dyn Fft<f64>>,
}

impl FFT {
    /// Creates a new FFT object for data of the specified length.
    pub fn new(len: usize) -> Self {
        let mut planner = FftPlanner::new();

        // Creates Fft objects for both the forward and inverse transform
        let forward_fft = planner.plan_fft_forward(len);
        let inverse_fft = planner.plan_fft_inverse(len);

        FFT { forward_fft, inverse_fft }
    }

    /// Processes the forward FFT of given data in-place.
    pub fn forward(&self, data: &mut [Complex<f64>]) {
        self.forward_fft.process(data);
    }

    /// Processes the inverse FFT of given data in-place.
    pub fn inverse(&self, data: &mut [Complex<f64>]) {
        self.inverse_fft.process(data);

        // Calculates the normalisation factor
        let normalisation_factor = 1. / data.len() as f64;

        // Normalises each element in the data
        for c in data {
            c.re *= normalisation_factor;
            c.im *= normalisation_factor;
        }
    }
}