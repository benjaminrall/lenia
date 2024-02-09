use crate::ALPHA;

/// A structure that stores the parameters necessary to create a Lenia system.
pub struct LeniaParameters {
    // Growth function parameters
    pub mu: f64,
    pub sigma: f64,

    // Kernel parameters
    pub alpha: i32,
    pub betas: Vec<f64>,

    // Space and time parameters
    pub kernel_radius: usize,
    pub dt: f64,
}

impl LeniaParameters {
    /// The basic set of Lenia parameters for a unimodal kernel with default alpha.
    pub fn basic(mu: f64, sigma: f64, kernel_radius: usize, dt: f64) -> Self {
        LeniaParameters {
            mu, sigma, alpha: ALPHA, betas: vec![1.], kernel_radius, dt,
        }
    }

    /// An extended set of Lenia parameters for a multimodal kernel with default alpha.
    pub fn extended(mu: f64, sigma: f64, kernel_radius: usize,
                    dt: f64, betas: Vec<f64>) -> Self {
        LeniaParameters {
            mu, sigma, alpha: ALPHA, betas, kernel_radius, dt,
        }
    }

    /// The full set of Lenia parameters for a multimodal kernel with custom alpha.
    pub fn full(mu: f64, sigma: f64, kernel_radius: usize,
                dt: f64, betas: Vec<f64>, alpha: i32) -> Self {
        LeniaParameters {
            mu, sigma, alpha, betas, kernel_radius, dt,
        }
    }
}

