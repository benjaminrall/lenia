use crate::parameters::LeniaParameters;

pub trait GrowthFunctionTrait: Sync {
    fn call(&self, n: f64) -> f64;

    fn from_params(params: &LeniaParameters) -> GrowthFunction where Self: Sized;

    fn recreate(&self, params: &LeniaParameters) -> GrowthFunction;
}

pub struct GrowthFunction(pub Box<dyn GrowthFunctionTrait>);

impl GrowthFunction {
    pub fn call(&self, n: f64) -> f64 {
        self.0.call(n)
    }

    pub fn recreate(&self, params: &LeniaParameters) -> GrowthFunction {
        self.0.recreate(params)
    }
}