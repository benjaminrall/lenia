use ndarray::AssignElem;
use pixel_canvas::Canvas;
use pixel_canvas::input::glutin::event::MouseButton;
use crate::display::LeniaState;
use crate::systems::{System, SystemTrait};

/// Displays a Lenia system on a pixel canvas and provides ways to interact with it
pub struct LeniaDisplay {
    system: System,
    scale: usize,
}

impl LeniaDisplay {
    /// Creates a new display for a given Lenia system.
    pub fn new(system: System) -> LeniaDisplay {
        LeniaDisplay { system, scale: 1 }
    }

    /// Creates a new scaled display for a given Lenia system.
    pub fn new_scaled(system: System, scale: usize) -> LeniaDisplay {
        LeniaDisplay { system, scale }
    }

    /// Renders the Lenia system as a pixel canvas window.
    pub fn render(mut self) {
        let height = self.system.get_height() * self.scale;
        let width = self.system.get_width() * self.scale;

        let canvas = Canvas::new(width, height)
            .title("Lenia")
            .state(LeniaState::new())
            .input(LeniaState::handle_event)
            .show_ms(true);

        let mut simulating = true;

        let draw_size = 5 * self.scale;

        canvas.render(move |state, image| {
            // Handles keyboard input
            if let Some(key) = state.keyboard.key_pressed {
                match key {
                    ' ' => self.system.0.iterate(),
                    's' => simulating = !simulating,
                    'r' => self.system.0.reset(),
                    _ => {}
                }

                state.keyboard.reset();
            }

            // Handles mouse input
            if let Some(button) = state.mouse_input.button_pressed {
                if button == MouseButton::Left {
                    let x = state.mouse_movement.x as usize;
                    let y = state.mouse_movement.y as usize;

                    for iy in (y - draw_size..=y + draw_size).step_by(self.scale) {
                        for ix in (x - draw_size..=x + draw_size).step_by(self.scale) {
                            let mut row = iy;
                            let mut col = ix;

                            if self.scale > 1 {
                                row /= self.scale;
                                col /= self.scale;
                            }

                            self.system.0.set_cell(0, &[row, col], 1.);
                        }
                    }
                }
            }

            // Simulates the Lenia system
            if simulating {
                self.system.0.iterate();
            }

            // Sets each pixel in the image
            let width = image.width();
            for (y, row) in image.chunks_mut(width).enumerate() {
                for (x, pixel) in row.iter_mut().enumerate() {
                    if self.scale > 1 {
                        let iy = y / self.scale;
                        let ix = x / self.scale;

                        *pixel = self.system.0.get_pixel(ix, iy)
                    } else {
                        *pixel = self.system.0.get_pixel(x, y)
                    }
                }
            }
        });
    }
}