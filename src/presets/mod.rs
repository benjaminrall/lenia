// Module definition
pub mod default_lenia;
pub mod game_of_life;
pub mod smooth_life;

// Re-imported structs for simpler paths
pub use self::default_lenia::DefaultLeniaPreset;
pub use self::smooth_life::SmoothLifePreset;
pub use self::game_of_life::GameOfLifePreset;