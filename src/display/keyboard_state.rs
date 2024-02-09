use pixel_canvas::canvas::{CanvasInfo};
use pixel_canvas::input::{Event, WindowEvent};

/// An input handler that tracks characters pressed on the keyboard.
pub struct KeyboardState {
    pub key_pressed: Option<char>
}

impl KeyboardState {
    /// Create a KeyboardState. For use with the `state` method.
    pub fn new() -> Self { KeyboardState { key_pressed: None } }

    /// Handle input for the keyboard. For use with the `input` method.
    pub fn handle_input(_info: &CanvasInfo, keyboard: &mut KeyboardState, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(key),
                ..
            } => {
                keyboard.key_pressed = Some(*key);
                true
            },
            _ => false
        }
    }

    /// Resets the current keyboard state.
    pub fn reset(&mut self) {
        self.key_pressed = None;
    }
}