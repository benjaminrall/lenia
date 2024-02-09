use crate::growth_functions::{GrowthFunction, GrowthFunctionTrait};
use crate::LeniaParameters;

pub struct GaussianGrowth {
    // Centre of the function
    mu: f64,

    // Reciprocal of 2 * the variance of the function
    k_recip: f64
}

impl GaussianGrowth {
    pub fn new(mu: f64, sigma: f64) -> GrowthFunction {
        GrowthFunction(Box::new(GaussianGrowth {
            mu, k_recip: 1. / (2. * sigma * sigma)
        }))
    }
}

impl GrowthFunctionTrait for GaussianGrowth {
    fn call(&self, n: f64) -> f64 {
        let l = n - self.mu;
        2. * (-l * l * self.k_recip).exp() - 1.
    }

    fn from_params(params: &LeniaParameters) -> GrowthFunction {
        GaussianGrowth::new(params.mu, params.sigma)
    }

    fn recreate(&self, params: &LeniaParameters) -> GrowthFunction {
        GaussianGrowth::from_params(params)
    }
}