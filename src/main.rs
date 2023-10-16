use pixel_canvas::{Canvas, Color};
use pixel_canvas::input::MouseState;

fn main() {
    let canvas = Canvas::new(512, 512)
        .title("Test")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    canvas.render(|mouse, image| {
        let width = image.width();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let dx = x as i32 - mouse.x;
                let dy = y as i32 - mouse.y;
                let dist = dx * dx + dy * dy;
                *pixel = Color {
                    r: if dist < 128 * 128 { dy as u8 } else { 0 },
                    g: if dist < 128 * 128 { dx as u8 } else { 0 },
                    b: 0
                }
            }
        }
    });
}
