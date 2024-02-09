use crate::LeniaParameters;
use crate::kernels::Kernel;

pub trait KernelCoreTrait {
    fn core(&self, r: f64) -> f64;

    fn from_params(
        params: &LeniaParameters,
        channel_shape: Vec<usize>
    ) -> Kernel where Self: Sized;

    fn recreate(&self, params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel;
}
pub struct KernelCore(pub Box<dyn KernelCoreTrait>);

impl KernelCore {
    pub fn core(&self, r: f64) -> f64 {
        self.0.core(r)
    }
}
