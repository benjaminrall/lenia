// Module definition
pub mod lenia_display;
pub mod lenia_state;
pub mod keyboard_state;
pub mod mouse_input_state;

// Re-imported structs for simpler paths
pub use self::lenia_display::LeniaDisplay;
pub use self::lenia_state::LeniaState;
pub use self::keyboard_state::KeyboardState;
pub use self::mouse_input_state::MouseInputState;
