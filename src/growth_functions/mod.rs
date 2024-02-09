// Module definitions
pub mod growth_function;
pub mod gaussian;
pub mod polynomial;
pub mod trapezoidal;
pub mod step;

// Re-imported structs for simpler paths
pub use self::growth_function::GrowthFunction;
pub use self::growth_function::GrowthFunctionTrait;
pub use self::gaussian::GaussianGrowth;
pub use self::polynomial::PolynomialGrowth;
pub use self::trapezoidal::TrapezoidalGrowth;
pub use self::step::StepGrowth;