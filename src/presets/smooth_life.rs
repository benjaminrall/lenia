use crate::LeniaParameters;
use crate::growth_functions::{GrowthFunctionTrait, GaussianGrowth};
use crate::kernels::{StepCore, KernelCoreTrait};
use crate::systems::{StandardLeniaSystem};
use crate::systems::system::System;

pub struct SmoothLifePreset;

impl SmoothLifePreset {
    pub fn new(shape: Vec<usize>) -> System {
        let params = LeniaParameters::basic(
            0.31, 0.049, 13, 1.
        );

        let kernel = StepCore::from_params(&params, shape.clone());
        let growth = GaussianGrowth::from_params(&params);

       StandardLeniaSystem::new(shape, kernel, growth, params)
    }
}