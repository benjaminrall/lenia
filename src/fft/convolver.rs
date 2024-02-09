use crate::fft::NDArrayFFT;
use crate::growth_functions::GrowthFunction;
use crate::kernels::Kernel;
use ndarray::{ArrayD, Zip};
use rustfft::num_complex::Complex;
use crate::LeniaParameters;

/// A structure that performs the convolution of a
/// pre-defined kernel over an input.
pub struct Convolver {
    kernel: Kernel,
    growth: GrowthFunction,
    fft: NDArrayFFT,
    result: ArrayD<Complex<f64>>,
}

impl Convolver {
    /// Creates a new convolver for a given kernel and growth function
    pub fn new(kernel: Kernel, growth: GrowthFunction) -> Self {
        let shape = kernel.get_channel_shape();

        Convolver {
            kernel,
            growth,
            fft: NDArrayFFT::new(&shape),
            result: ArrayD::zeros(shape),
        }
    }

    /// Computes the convolution of the kernel over the input data
    pub fn compute(&mut self, data: &mut ArrayD<f64>, dt: f64) {
        // Creates an array used for computing the convolution
        self.result.zip_mut_with(data, |a, &b| { a.re = b; a.im = 0.; });

        // Calculates the convolution of the kernel over the input data
        self.fft.forward_par(&mut self.result);
        self.result.zip_mut_with(self.kernel.get_fft(), |a, b| {
            let im = (a.re * b.im) + (a.im * b.re);
            a.re = (a.re * b.re) - (a.im * b.im);
            a.im = im;
        });
        self.fft.inverse_par(&mut self.result);

        // Applies the result to the input data
        Zip::from(data).and(&self.result).par_for_each(
            |a, b| {
                let g = self.growth.call(b.re);
                *a = (*a + g * dt).clamp(0., 1.);
            }
        );
    }

    /// Recreates a convolver for a given set of parameters
    pub fn recreate(&self, params: &LeniaParameters) -> Self {
        let shape = self.result.shape().to_vec();

        Convolver {
            kernel: self.kernel.recreate(params),
            growth: self.growth.recreate(params),
            fft: NDArrayFFT::new(&shape),
            result: ArrayD::zeros(shape),
        }
    }
}