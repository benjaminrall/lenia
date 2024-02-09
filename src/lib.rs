// Module definition
pub mod fft;
pub mod kernels;
pub mod growth_functions;
pub mod parameters;
pub mod systems;
pub mod channel;
pub mod presets;
pub mod display;

// Re-imported structs for simpler paths
pub use self::parameters::LeniaParameters;
pub use self::channel::Channel;

// Library constants
pub const ALPHA: i32 = 4;
