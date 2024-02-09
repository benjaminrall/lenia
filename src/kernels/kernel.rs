
use ndarray::ArrayD;
use rustfft::num_complex::Complex;
use crate::LeniaParameters;
use crate::kernels::KernelCore;
use crate::kernels::utils::{euclidean_distance, normalise, transform_kernel};


pub struct Kernel {
    core: KernelCore,
    channel_shape: Vec<usize>,
    fft: ArrayD<Complex<f64>>
}

impl Kernel {
    pub fn unimodal(core: KernelCore, channel_shape: Vec<usize>, radius: usize) -> Self {
        Kernel::multimodal(core, channel_shape, radius, &vec![1.0])
    }

    pub fn multimodal(core: KernelCore, channel_shape: Vec<usize>, radius: usize, betas: &Vec<f64>) -> Kernel {
        // Stores the number of peaks and dimensions for the kernel
        let peaks = betas.len();
        let peaks_float = peaks as f64;
        let dimensions = channel_shape.len();

        // Calculates the kernel shape and kernel centre as vectors
        let kernel_size = 2 * radius + 1;
        let kernel_centre = vec![radius as f64; dimensions];
        let kernel_shape = vec![kernel_size; dimensions];
        let output_shape = channel_shape.clone();

        // Creates a mutable vector used to store the current index information
        let mut index_vector = vec![0.; dimensions];

        // Used to normalise the position of indices
        let normaliser = 1. / radius as f64;

        // Creates the base kernel using the core function
        let base = ArrayD::from_shape_fn(kernel_shape, |index| {
            // Converts the current index into a vector containing the indices in each dimension
            for i in 0..dimensions {
                index_vector[i] = index[i] as f64;
            }

            // Calculates the distance of the index from the kernel centre
            let mut r = euclidean_distance(&index_vector, &kernel_centre) * normaliser;

            // Calculates which peak the index belongs to and scales `r` accordingly
            let mut beta = 1.;
            if peaks > 1 {
                for n in 1..=peaks {
                    if r < n as f64 / peaks_float {
                        r = peaks_float * r - n as f64 + 1.;
                        beta = betas[n - 1];
                        break;
                    }
                }
            }

            // Calculates the value for the index using the core function
            beta * core.core(r)
        });

        // Normalises the kernel to ensure K * A ∈ [0,1]
        let normalised = normalise(&base);

        // Computes the kernel's fast fourier transform for the given channel shape
        let fft = transform_kernel(&normalised, output_shape);

        // Creates and returns the kernel instance
        Kernel { core, channel_shape, fft }
    }

    pub fn get_fft(&self) -> &ArrayD<Complex<f64>> {
        &self.fft
    }

    pub fn get_channel_shape(&self) -> Vec<usize> {
        self.channel_shape.clone()
    }

    pub fn recreate(&self, params: &LeniaParameters) -> Kernel {
        self.core.0.recreate(params, self.channel_shape.clone())
    }
}