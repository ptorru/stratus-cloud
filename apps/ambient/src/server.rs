use tonic::{transport::Server, Request, Response, Status};

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloResponse, HelloRequest};

use bme280::BME280;

use linux_embedded_hal as hal;


use embedded_graphics::{
    mono_font::{ascii::FONT_6X13, ascii::FONT_4X6, ascii::FONT_9X15_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::{Baseline, Text},
};
use hal::{Delay, I2cdev};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

struct Data {
    temp: f32,
    pres: f32,
    humi: f32,
}

fn get_data() -> Data {

    // using Linux I2C Bus #1 in this example
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();

    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);

    // initialize the sensor
    bme280.init().unwrap();

    // measure temperature, pressure, and humidity
    let measurements = bme280.measure().unwrap();

    Data {
        temp: measurements.temperature,
        pres: measurements.pressure,
        humi: measurements.humidity,
    }
}

fn draw_screen<S>(msg: S, data: &Data)
where
    S: AsRef<str>,
{
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    disp.init().unwrap();

    let offset = 4;
    let xoffset = 100;
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Size::new(127, 63))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    let text_title = MonoTextStyleBuilder::new()
        .font(&FONT_9X15_BOLD)
        .text_color(BinaryColor::On)
        .build();

    let text_mid = MonoTextStyleBuilder::new()
        .font(&FONT_6X13)
        .text_color(BinaryColor::On)
        .build();

    let text_small = MonoTextStyleBuilder::new()
        .font(&FONT_4X6)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(
        &format!("{}", msg.as_ref()),
        Point::new(offset, offset),
        text_small,
        Baseline::Top,
    )
    .draw(&mut disp)
    .unwrap();

    Text::with_baseline(
        &format!("Tp\nPr\nHu"),
        Point::new(offset, 14),
        text_title,
        Baseline::Top,
    )
    .draw(&mut disp)
    .unwrap();

    Text::with_baseline(
        &format!("{}\n{}\n{}", data.temp, data.pres, data.humi),
        Point::new(offset+22, offset+14),
        text_mid,
        Baseline::Top,
    )
    .draw(&mut disp)
    .unwrap();

    // circle
    Circle::new(Point::new(xoffset, offset), 16)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // square
    Rectangle::new(Point::new(xoffset, offset + 18), Size::new_equal(16))
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();

    // triangle
    Triangle::new(
        Point::new(xoffset, 16 + offset + 37),
        Point::new(xoffset + 16, 16 + offset + 37),
        Point::new(xoffset + 8, offset + 37),
    )
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();


    disp.flush().unwrap();

}

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGreeter {
}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Received request from: {:?}", request);

        // Add here the code to read from the sensor
        let amb_data = get_data();
        let message = request.into_inner();
        draw_screen(format!("{:?}", message.name), &amb_data);

        let response = greeter::HelloResponse {
            message: format!("Hello {}!", message.name).into(),
            temp: amb_data.temp,
            pres: amb_data.pres,
            humi: amb_data.humi,
        };

        Ok(Response::new(response))
    }
}

// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:2424".parse()?;
    let greeter = MyGreeter::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
