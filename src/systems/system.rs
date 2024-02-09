use pixel_canvas::Color;
use crate::LeniaParameters;

pub trait SystemTrait {
    fn iterate(&mut self);

    fn reset(&mut self);

    fn get_pixel(&self, x: usize, y: usize) -> Color;

    fn get_width(&self) -> usize;

    fn get_height(&self) -> usize;

    fn set_cell(&mut self, channel: usize, index: &[usize], value: f64);

    fn get_params(&self) -> &LeniaParameters;

    fn set_params(&mut self, params: LeniaParameters);
}

pub struct System(pub Box<dyn SystemTrait>);

impl SystemTrait for System {
    fn iterate(&mut self) {
        self.0.iterate()
    }

    fn reset(&mut self) {
        self.0.reset()
    }

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.0.get_pixel(x, y)
    }

    fn get_width(&self) -> usize {
        self.0.get_width()
    }

    fn get_height(&self) -> usize {
        self.0.get_height()
    }

    fn set_cell(&mut self, channel: usize, index: &[usize], value: f64) {
        self.0.set_cell(channel, index, value)
    }

    fn get_params(&self) -> &LeniaParameters {
        self.0.get_params()
    }

    fn set_params(&mut self, params: LeniaParameters) {
        self.0.set_params(params)
    }
}