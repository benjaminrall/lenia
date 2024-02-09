use pixel_canvas::Color;
use crate::{Channel, LeniaParameters};
use crate::fft::Convolver;
use crate::growth_functions::GrowthFunction;
use crate::kernels::Kernel;
use crate::systems::{System, SystemTrait};

pub struct StandardLeniaSystem {
    params: LeniaParameters,
    channel: Channel,
    convolver: Convolver,
}

impl StandardLeniaSystem {
    pub fn new(
        shape: Vec<usize>,
        kernel: Kernel,
        growth: GrowthFunction,
        params: LeniaParameters
    ) -> System {
        let convolver = Convolver::new(kernel, growth);
        let channel = Channel::new(shape);

        System(Box::new(StandardLeniaSystem {
            params, channel, convolver
        }))
    }
}

impl SystemTrait for StandardLeniaSystem {
    fn iterate(&mut self) {
        self.convolver.compute(self.channel.mut_field_ref(), self.params.dt)
    }

    fn reset(&mut self) {
        self.channel.reset()
    }

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        let v = (255. * self.channel.get_cell(&[y, x])) as u8;

        Color { r: v, g: v, b: v }
    }

    fn get_width(&self) -> usize {
        self.channel.field_ref().shape().to_vec()[1]
    }

    fn get_height(&self) -> usize {
        self.channel.field_ref().shape().to_vec()[0]
    }

    fn set_cell(&mut self, _channel: usize, index: &[usize], value: f64) {
        self.channel.set_cell(index, value)
    }

    fn get_params(&self) -> &LeniaParameters {
        &self.params
    }

    fn set_params(&mut self, params: LeniaParameters) {
        self.convolver = self.convolver.recreate(&params);
        self.params = params;
    }
}