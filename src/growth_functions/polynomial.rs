use crate::{ALPHA, LeniaParameters};
use crate::growth_functions::{GrowthFunction, GrowthFunctionTrait};


pub struct PolynomialGrowth {
    // Centre of the function
    mu: f64,

    // The lower width of the function
    k: f64,

    // The reciprocal of the square of the lower width
    k_squared_recip: f64,

    // Degree of the polynomial
    alpha: i32
}

impl PolynomialGrowth {
    pub fn new(mu: f64, sigma: f64) -> GrowthFunction {
        let k = 3. * sigma;

        GrowthFunction(Box::new(PolynomialGrowth {
            mu,
            alpha: ALPHA,
            k,
            k_squared_recip: 1. / (k * k),
        }))
    }

    pub fn new_with_alpha(mu: f64, sigma: f64, alpha: i32) -> GrowthFunction {
        let k = 3. * sigma;

        GrowthFunction(Box::new(PolynomialGrowth {
            mu,
            alpha,
            k,
            k_squared_recip: 1. / (k * k),
        }))
    }
}

impl GrowthFunctionTrait for PolynomialGrowth {
    fn call(&self, n: f64) -> f64 {
        let l = (n - self.mu).abs();

        if l <= self.k {
            2. * (1. - l * l * self.k_squared_recip).powi(self.alpha) - 1.
        } else {
            -1.
        }
    }

    fn from_params(params: &LeniaParameters) -> GrowthFunction {
        PolynomialGrowth::new_with_alpha(params.mu, params.sigma, params.alpha)
    }

    fn recreate(&self, params: &LeniaParameters) -> GrowthFunction {
        PolynomialGrowth::from_params(params)
    }
}