use crate::fft::FFT;
use ndarray::{ArrayD, Axis, Zip};
use rayon::prelude::*;
use rustfft::num_complex::Complex;

/// A structure that plans both the forward and inverse transforms for an
/// n-dimensional array of a specified shape, allowing efficient FFT computation.
pub struct NDArrayFFT { ffts: Vec<FFT> }

impl NDArrayFFT {
    /// Creates a new planned FFT object for an NDArray of the specified shape.
    pub fn new(shape: &Vec<usize>) -> Self {
        let mut ffts: Vec<FFT> = vec![];

        // Creates a vector containing a planned FFT for each dimension of the shape
        for &size in shape {
            ffts.push(FFT::new(size));
        }

        NDArrayFFT { ffts }
    }

    /// Computes the transform for a given axis of an input NDArray.
    pub fn compute_axis(&mut self, data: &mut ArrayD<Complex<f64>>, axis: Axis, inverse: bool) {
        for mut lane in data.lanes_mut(axis) {
            // Converts the lane to a buffer to use for the FFT
            let mut buffer = lane.to_vec();

            // Calculates the transform for the data buffer
            if inverse {
                self.ffts[axis.0].inverse(&mut buffer);
            } else {
                self.ffts[axis.0].forward(&mut buffer);
            }

            // Sets the lane values to equal the buffer output
            lane.iter_mut().zip(&buffer).for_each(|(l, &b)| *l = b);
        }
    }

    /// Processes the forward FFT for a given NDArray.
    pub fn forward(&mut self, data: &mut ArrayD<Complex<f64>>) {
        // Calculates the transform for each axis
        for axis in 0..data.shape().len() {
            self.compute_axis(data, Axis(axis), false);
        }
    }

    /// Processes the inverse FFT for a given NDArray.
    pub fn inverse(&mut self, data: &mut ArrayD<Complex<f64>>) {
        // Calculates the inverse transform for each axis in reverse
        for axis in (0..data.shape().len()).rev() {
            self.compute_axis(data, Axis(axis), true);
        }
    }

    /// Computes the transform for a given axis of an input NDArray using parallelisation.
    pub fn compute_axis_par(&mut self, data: &mut ArrayD<Complex<f64>>, axis: Axis, inverse: bool) {
        // Calculates the transform for each lane in the axis in parallel
        let fft = &self.ffts[axis.0];
        Zip::from(data.lanes_mut(axis))
            .into_par_iter()
            .for_each(|mut lane| {
                // Converts the lane to a buffer to use for the FFT
                let mut buffer = lane.0.to_vec();

                // Calculates the transform for the data buffer
                if inverse {
                    fft.inverse(&mut buffer);
                } else {
                    fft.forward(&mut buffer);
                }

                // Sets the lane values to equal the buffer output
                lane.0.iter_mut().zip(&buffer).for_each(|(l, &b)| *l = b);
            });
    }

    /// Processes the forward FFT for a given NDArray in parallel.
    pub fn forward_par(&mut self, data: &mut ArrayD<Complex<f64>>) {
        // Calculates the transform for each axis
        for axis in 0..data.shape().len() {
            self.compute_axis_par(data, Axis(axis), false);
        }
    }

    /// Processes the inverse FFT for a given NDArray in inverse.
    pub fn inverse_par(&mut self, data: &mut ArrayD<Complex<f64>>) {
        // Calculates the inverse transform for each axis in reverse
        for axis in (0..data.shape().len()).rev() {
            self.compute_axis_par(data, Axis(axis), true);
        }
    }
}