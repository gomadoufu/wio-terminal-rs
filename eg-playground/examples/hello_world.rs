use embedded_graphics::{
    fonts::{Font12x16, Text},
    pixelcolor::Rgb565,
    prelude::*,
    style::TextStyle,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("hello world", &output_settings);

    Text::new("Hello World!", Point::new(0, 0))
        .into_styled(TextStyle::new(Font12x16, Rgb565::GREEN))
        .draw(&mut display)?;

    window.show_static(&display);
    Ok(())
}
