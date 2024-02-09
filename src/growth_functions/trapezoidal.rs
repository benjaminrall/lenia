use crate::growth_functions::{GrowthFunction, GrowthFunctionTrait};
use crate::LeniaParameters;

pub struct TrapezoidalGrowth {
    // Centre of the function
    mu: f64,

    // Half the upper width of the function
    p: f64,

    // Half the lower width of the function
    q: f64,

    // Reciprocal of the difference between the lower and upper width
    width_diff_recip: f64
}

impl TrapezoidalGrowth {
    pub fn new(mu: f64, sigma: f64) -> GrowthFunction {
        let p = sigma / 2.;
        let q = sigma * 2.;

        GrowthFunction(Box::new(TrapezoidalGrowth {
            mu, p, q, width_diff_recip: 1. / (q - p)
        }))
    }
}

impl GrowthFunctionTrait for TrapezoidalGrowth {
    fn call(&self, n: f64) -> f64 {
        let l = (n - self.mu).abs();

        if l <= self.p {
            1.
        } else if l <= self.q {
            2. * (self.q - l) * self.width_diff_recip - 1.
        } else {
            -1.
        }
    }

    fn from_params(params: &LeniaParameters) -> GrowthFunction {
        TrapezoidalGrowth::new(params.mu, params.sigma)
    }

    fn recreate(&self, params: &LeniaParameters) -> GrowthFunction {
        TrapezoidalGrowth::from_params(params)
    }
}