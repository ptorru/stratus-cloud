// This example is based on the examples by:
// jamwaffles at: https://github.com/jamwaffles/ssd1306
// fmckeogh at: https://github.com/fmckeogh/ssd1306-raspi-examples

extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate ssd1306;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::{Baseline, Text},
};
use hal::I2cdev;
//use embedded_graphics::fonts::Font6x8;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    disp.init().unwrap();

    let yoffset = 4;
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::ncargo run --bin helloworld-cliente)
        .draw(&mut disp)
        .unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    // square
    Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(88, yoffset), 16)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    Text::with_baseline(
        &format!("Hello Pi!"),
        Point::new(20, 40),
        text_style,
        Baseline::Top,
    )
    .draw(&mut disp)
    .unwrap();

    disp.flush().unwrap();
}
