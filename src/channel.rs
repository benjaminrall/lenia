use ndarray::ArrayD;

pub struct Channel {
    field: ArrayD<f64>,
}

impl Channel {
    pub fn new(shape: Vec<usize>) -> Self {
        let field = ArrayD::zeros(shape.clone());
        Channel { field }
    }

    pub fn reset(&mut self) {
        self.field = ArrayD::zeros(self.field.shape());
    }

    pub fn set_cell(&mut self, index: &[usize], value: f64) {
        self.field[index] = value
    }

    pub fn get_cell(&self, index: &[usize]) -> f64 {
        self.field[index]
    }

    pub fn mut_field_ref(&mut self) -> &mut ArrayD<f64> {
        &mut self.field
    }

    pub fn field_ref(&self) -> &ArrayD<f64> {
        &self.field
    }
}