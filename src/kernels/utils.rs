use ndarray::{ArrayD, Axis, Slice};
use rustfft::num_complex::Complex;
use crate::fft::NDArrayFFT;

/// Returns the euclidean distance between 2 N-dimensional points.
pub fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let mut out = 0.;
    for i in 0..a.len() {
        out += (a[i] - b[i]).powi(2);
    }
    out.sqrt()
}

/// Normalises a kernel so that all elements add to one.
pub fn normalise(kernel: &ArrayD<f64>) -> ArrayD<f64> {
    let kernel_sum = kernel.sum();
    let normalisation_factor = if kernel_sum != 0. { 1.0 / kernel.sum() } else { 1. };
    kernel.map(|&a| a * normalisation_factor)
}

/// Expands the given array to match the given shape.
pub fn expand(kernel: &ArrayD<f64>, shape: Vec<usize>) -> ArrayD<f64>
{
    // Creates output array
    let mut output: ArrayD<f64> = ArrayD::zeros(shape);
    let original_shape = kernel.shape();

    // Creates a view of the overlap between the original array and output array
    let mut overlap = output.view_mut();
    for axis in 0..original_shape.len() {
        overlap.slice_axis_inplace(Axis(axis), Slice::from(0..original_shape[axis]));
    }

    // Copy the data from the original array.
    overlap.assign(kernel);

    output
}

/// Shifts a given axis in the array to the left by a specified amount.
pub fn shift_axis(arr: &ArrayD<f64>, axis: Axis, shift: usize) -> ArrayD<f64> {
    let mut shifted: ArrayD<f64> = ArrayD::zeros(arr.dim());

    for indices in arr.indexed_iter() {
        let mut new_indices = indices.0;

        new_indices[axis.0] = (new_indices[axis.0] + arr.shape()[axis.0] - shift) % arr.len_of(axis);

        shifted[new_indices] = *indices.1;
    }

    shifted
}

/// Shifts an array in several axes by several shifts at once.
pub fn shift(arr: ArrayD<f64>, axes: Vec<Axis>, shifts: Vec<usize>) -> ArrayD<f64> {
    let mut current_array = arr;

    for (&axis, shift) in axes.iter().zip(shifts) {
        current_array = shift_axis(&current_array, axis, shift);
    }

    current_array
}

/// Transforms a kernel so that it's ready for convolution with channels of the given shape
pub fn transform_kernel(array: &ArrayD<f64>, output_shape: Vec<usize>) -> ArrayD<Complex<f64>> {
    let mut fft = NDArrayFFT::new(&output_shape);

    let (axes, shifts): (Vec<Axis>, Vec<usize>) = array
        .shape()
        .iter()
        .enumerate()
        .map(|(i, &dim)| (Axis(i), dim / 2))
        .unzip();

    let shifted = shift(expand(&array, output_shape), axes, shifts);
    let mut transformed = shifted.map(|&a| Complex::new(a, 0.0));

    fft.forward_par(&mut transformed);

    transformed
}