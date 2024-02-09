use pixel_canvas::canvas::{CanvasInfo};
use pixel_canvas::input::{Event, MouseState};
use crate::display::KeyboardState;
use crate::display::MouseInputState;

/// An input handler that tracks all events for the Lenia display.
pub struct LeniaState {
    pub keyboard: KeyboardState,
    pub mouse_movement: MouseState,
    pub mouse_input: MouseInputState,
}

impl LeniaState {
    /// Create a LeniaState. For use with the `state` method.
    pub fn new() -> Self {
        LeniaState {
            keyboard: KeyboardState::new(),
            mouse_movement: MouseState::new(),
            mouse_input: MouseInputState::new(),
        }
    }

    /// Handles all inputs for the current state. For use with the `input` method.
    pub fn handle_event(info: &CanvasInfo, state: &mut LeniaState, event: &Event<()>) -> bool {
        KeyboardState::handle_input(info, &mut state.keyboard, event)
            | MouseState::handle_input(info, &mut state.mouse_movement, event)
            | MouseInputState::handle_input(info, &mut state.mouse_input, event)
    }

    /// Resets the current keyboard state.
    pub fn reset(&mut self) {
        self.keyboard.key_pressed = None;
        self.mouse_input.button_pressed = None;
    }
}