use crate::{ALPHA, LeniaParameters};
use crate::kernels::{Kernel, KernelCore, KernelCoreTrait};

pub struct GaussianCore {
    alpha: i32
}

impl GaussianCore {
    pub fn new() -> KernelCore {
        KernelCore(Box::new(GaussianCore { alpha: ALPHA }))
    }

    pub fn new_with_alpha(alpha: i32) -> KernelCore {
        KernelCore(Box::new(GaussianCore { alpha }))
    }
}

impl KernelCoreTrait for GaussianCore {
    fn core(&self, r: f64) -> f64 {
        if r <= 0. || r >= 1. {
            return 0.;
        }

        let k = 4. * r * (1. - r);

        (self.alpha as f64 * (1. - 1. / k)).exp()
    }

    fn from_params(params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        Kernel::multimodal(
            GaussianCore::new_with_alpha(params.alpha),
            channel_shape, params.kernel_radius, &params.betas
        )
    }

    fn recreate(&self, params: &LeniaParameters, channel_shape: Vec<usize>) -> Kernel {
        GaussianCore::from_params(params, channel_shape)
    }
}