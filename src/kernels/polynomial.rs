use crate::{ALPHA, LeniaParameters};
use crate::kernels::{Kernel, KernelCore, KernelCoreTrait};

pub struct PolynomialCore {
    alpha: i32
}

impl PolynomialCore {
    pub fn new() -> KernelCore {
        KernelCore(Box::new(PolynomialCore { alpha: ALPHA }))
    }

    pub fn new_with_alpha(alpha: i32) -> KernelCore {
        KernelCore(Box::new(PolynomialCore { alpha }))
    }
}

impl KernelCoreTrait for PolynomialCore {
    fn core(&self, r: f64) -> f64 {
        let k = 4. * r * (1. - r);

        k.powi(self.alpha)
    }

    fn from_params(params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        Kernel::multimodal(
            PolynomialCore::new_with_alpha(params.alpha),
            channel_shape, params.kernel_radius, &params.betas
        )
    }

    fn recreate(&self, params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        PolynomialCore::from_params(params, channel_shape)
    }
}