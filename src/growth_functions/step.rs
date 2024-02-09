use crate::growth_functions::{GrowthFunction, GrowthFunctionTrait};
use crate::LeniaParameters;

pub struct StepGrowth {
    // Centre of the function
    mu: f64,

    // Half the width of the function
    sigma : f64
}

impl StepGrowth {
    pub fn new(mu: f64, sigma: f64) -> GrowthFunction {
        GrowthFunction(Box::new(StepGrowth {
            mu, sigma
        }))
    }
}

impl GrowthFunctionTrait for StepGrowth {
    fn call(&self, n: f64) -> f64 {
        let l = (n - self.mu).abs();

        if l <= self.sigma {
            1.
        } else {
            -1.
        }
    }

    fn from_params(params: &LeniaParameters) -> GrowthFunction {
        StepGrowth::new(params.mu, params.sigma)
    }

    fn recreate(&self, params: &LeniaParameters) -> GrowthFunction {
        StepGrowth::from_params(params)
    }
}