use crate::LeniaParameters;
use crate::growth_functions::{GaussianGrowth, GrowthFunctionTrait};
use crate::kernels::{GaussianCore, KernelCoreTrait};
use crate::systems::{StandardLeniaSystem, System};

pub struct DefaultLeniaPreset;

impl DefaultLeniaPreset {
    pub fn new(shape: Vec<usize>, size: usize) -> System {
        let params = LeniaParameters::basic(
            0.14, 0.015, size, 0.1
        );

        let kernel = GaussianCore::from_params(&params, shape.clone());
        let growth = GaussianGrowth::from_params(&params);

        StandardLeniaSystem::new(shape, kernel, growth, params)
    }
}