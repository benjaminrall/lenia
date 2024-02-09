use crate::LeniaParameters;
use crate::kernels::{Kernel, KernelCore, KernelCoreTrait};

pub struct StepCore;

impl StepCore {
    pub fn new() -> KernelCore {
        KernelCore(Box::new(StepCore { }))
    }
}

impl KernelCoreTrait for StepCore {
    fn core(&self, r: f64) -> f64 {
        if 0.25 <= r && r <= 0.75 {
            1.
        } else {
            0.
        }
    }

    fn from_params(params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        Kernel::multimodal(
            StepCore::new(),
            channel_shape, params.kernel_radius, &params.betas
        )
    }

    fn recreate(&self, params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        StepCore::from_params(params, channel_shape)
    }
}