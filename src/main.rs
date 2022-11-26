use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_hal::spi::MODE_3;
use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::*;
use esp_idf_hal::units::FromValueType;
use mipidsi::{Builder, Orientation};
use std::thread;
use std::time::Duration;

use u8g2_fonts::types::HorizontalAlignment;
use u8g2_fonts::{
    fonts,
    types::{FontColor, VerticalPosition},
    FontRenderer,
};

fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let mut vdd = PinDriver::output(peripherals.pins.gpio21)?;
    let rst = PinDriver::output(peripherals.pins.gpio40)?;
    let dc = PinDriver::output(peripherals.pins.gpio39)?;
    let mut backlight = PinDriver::output(peripherals.pins.gpio45)?;
    let sclk = peripherals.pins.gpio36;
    let sdo = peripherals.pins.gpio35;
    let sdi: Option<AnyIOPin> = None;
    let cs = peripherals.pins.gpio7;

    // power ST7789
    vdd.set_high()?;
    backlight.set_drive_strength(DriveStrength::I40mA)?;
    backlight.set_low()?;

    let mut delay = Ets;

    // configuring the spi interface, note that in order for the ST7789 to work, the data_mode needs to be set to MODE_3
    let config = config::Config::new()
        .baudrate(80.MHz().into())
        .data_mode(MODE_3);

    let device =
        SpiDeviceDriver::new_single(spi, sclk, sdo, sdi, Dma::Disabled, Some(cs), &config)?;

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(device, dc);

    // create driver
    let mut display = Builder::st7789_pico1(di)
        .with_display_size(135, 240)
        // set default orientation
        .with_orientation(Orientation::Landscape(true))
        // initialize
        .init(&mut delay, Some(rst))
        .unwrap();

    display.clear(Rgb565::BLACK).unwrap();

    let font = FontRenderer::new::<fonts::u8g2_font_logisoso16_tf>();

    let text = "ESP32-S3 Anemometer";

    font.render_aligned(
        text,
        display.bounding_box().center() - Point::new(115, 35),
        VerticalPosition::Baseline,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::RED),
        &mut display,
    )
    .unwrap();

    let text = "Wind: 14.1 m/s";
    let font = FontRenderer::new::<fonts::u8g2_font_helvB18_tf>();
    font.render_aligned(
        text,
        display.bounding_box().center() - Point::new(115, 0),
        VerticalPosition::Baseline,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::YELLOW),
        &mut display,
    )
    .unwrap();

    let text = "GPS: 14.5 m/s";
    let font = FontRenderer::new::<fonts::u8g2_font_helvB18_tf>();
    font.render_aligned(
        text,
        display.bounding_box().center() - Point::new(115, -30),
        VerticalPosition::Baseline,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::MAGENTA),
        &mut display,
    )
    .unwrap();

    let text = "IP: 192.168.100.102  FW: v0.38.21";
    let font = FontRenderer::new::<fonts::u8g2_font_t0_14_tf>();
    font.render_aligned(
        text,
        display.bounding_box().center() - Point::new(115, -60),
        VerticalPosition::Baseline,
        HorizontalAlignment::Left,
        FontColor::Transparent(Rgb565::WHITE),
        &mut display,
    )
    .unwrap();

    backlight.set_high()?;

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
