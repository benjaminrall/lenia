use crate::LeniaParameters;
use crate::growth_functions::{GrowthFunctionTrait, StepGrowth};
use crate::kernels::{HighStepCore, KernelCoreTrait};
use crate::systems::{System, StandardLeniaSystem};

pub struct GameOfLifePreset;

impl GameOfLifePreset {
    pub fn new(shape: Vec<usize>) -> System {
        let params = LeniaParameters::basic(
            0.35, 0.07, 2, 1.
        );

        let kernel = HighStepCore::from_params(&params, shape.clone());
        let growth = StepGrowth::from_params(&params);

        StandardLeniaSystem::new(shape, kernel, growth, params)
    }
}