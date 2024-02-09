// Module definition
pub mod system;
pub mod standard_lenia;
pub mod expanded_lenia;

// Re-imported structs for simpler paths
pub use self::system::System;
pub use self::system::SystemTrait;
pub use self::standard_lenia::StandardLeniaSystem;
pub use self::expanded_lenia::ExpandedLeniaSystem;

