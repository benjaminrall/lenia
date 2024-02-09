use crate::LeniaParameters;
use crate::kernels::{Kernel, KernelCore, KernelCoreTrait};

pub struct HighStepCore;

impl HighStepCore {
    pub fn new() -> KernelCore {
        KernelCore(Box::new(HighStepCore { }))
    }
}

impl KernelCoreTrait for HighStepCore {
    fn core(&self, r: f64) -> f64 {
        if r < 0.25 {
            0.5
        } else if r <= 0.75 {
            1.
        } else {
            0.
        }
    }

    fn from_params(params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        Kernel::multimodal(
            HighStepCore::new(),
            channel_shape, params.kernel_radius, &params.betas
        )
    }

    fn recreate(&self, params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        HighStepCore::from_params(params, channel_shape)
    }
}