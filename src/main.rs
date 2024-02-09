use lenia::display::LeniaDisplay;
use lenia::presets::{DefaultLeniaPreset, GameOfLifePreset};

fn main() {
    let channel_shape = vec![720, 1280];

    let system = DefaultLeniaPreset::new(channel_shape.clone(), 48);

    //let system = GameOfLifePreset::new(channel_shape);

    let display = LeniaDisplay::new_scaled(system, 1);

    display.render();
}
