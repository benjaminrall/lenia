// Module definition
pub mod fft;
pub mod ndarray_fft;
pub mod convolver;

// Re-imported structs for simpler paths
pub use self::fft::FFT;
pub use self::ndarray_fft::NDArrayFFT;
pub use self::convolver::Convolver;