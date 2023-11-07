use embedded_graphics::{image::ImageRawLE, pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use wio_splash::WioSplash;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(320, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Wio Terminal Splash", &output_settings);

    let raw = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86, 64);
    let splash = WioSplash::new(Rgb565::GREEN, raw);
    splash.draw(&mut display)?;

    window.show_static(&display);
    Ok(())
}
