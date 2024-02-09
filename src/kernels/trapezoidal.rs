use crate::LeniaParameters;
use crate::kernels::{Kernel, KernelCore, KernelCoreTrait};

pub struct TrapezoidalCore;

impl TrapezoidalCore {
    pub fn new() -> KernelCore {
        KernelCore(Box::new(TrapezoidalCore { }))
    }
}

impl KernelCoreTrait for TrapezoidalCore {
    fn core(&self, r: f64) -> f64 {
        if r <= 0.2 || r >= 0.8 {
            0.
        } else if r <= 0.4 {
            (r - 0.2) / 0.2
        } else if r <= 0.6 {
            1.
        } else {
            (0.8 - r) / 0.2
        }
    }

    fn from_params(params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        Kernel::multimodal(
            TrapezoidalCore::new(),
            channel_shape, params.kernel_radius, &params.betas
        )
    }

    fn recreate(&self, params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        TrapezoidalCore::from_params(params, channel_shape)
    }
}