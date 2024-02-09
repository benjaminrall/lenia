// Module definition
pub mod kernel;
pub mod kernel_core;
pub mod gaussian;
pub mod polynomial;
pub mod trapezoidal;
pub mod step;
pub mod high_step;
pub mod utils;

// Re-imported structs for simpler paths
pub use self::kernel::Kernel;
pub use self::kernel_core::KernelCore;
pub use self::kernel_core::KernelCoreTrait;
pub use self::gaussian::GaussianCore;
pub use self::polynomial::PolynomialCore;
pub use self::trapezoidal::TrapezoidalCore;
pub use self::step::StepCore;
pub use self::high_step::HighStepCore;