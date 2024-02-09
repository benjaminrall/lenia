use pixel_canvas::canvas::{CanvasInfo};
use pixel_canvas::input::{Event, WindowEvent};
use pixel_canvas::input::glutin::event::{ElementState, MouseButton};

/// An input handler that pressed mouse buttons.
pub struct MouseInputState {
    pub button_pressed: Option<MouseButton>
}

impl MouseInputState {
    /// Create a MouseInputState. For use with the `state` method.
    pub fn new() -> Self { MouseInputState { button_pressed: None } }

    /// Handle input for the mouse. For use with the `input` method.
    pub fn handle_input(_info: &CanvasInfo, mouse: &mut MouseInputState, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if *state == ElementState::Pressed {
                    mouse.button_pressed = Some(*button)
                } else {
                    mouse.button_pressed = None
                }
                true
            },
            _ => false
        }
    }

    /// Resets the current mouse input state.
    pub fn reset(&mut self) {
        self.button_pressed = None;
    }
}