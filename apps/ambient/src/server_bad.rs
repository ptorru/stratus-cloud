use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::{Baseline, Text},
};
use linux_embedded_hal::I2cdev;
//use embedded_graphics::fonts::Font6x8;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use tonic::{transport::Server, Request, Response, Status};
use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloResponse, HelloRequest};



// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("greeter");
}

pub struct MyGreeter<a, C, M, S, SIZE, MODE> {
    pub interface: I2cdev,
    pub yoffset: i16,
    pub style: PrimitiveStyleBuilder<C>,
    pub text_style: MonoTextStyleBuilder<a, M>,
    pub disp: Ssd1306<S, SIZE, MODE>,
}


impl <C, M, S, SIZE, MODE> MyGreeter<C, M, S, SIZE, MODE>
    where
    C:,
    M:,
    S:,
    SIZE: ,
    MODE: ,
{
    pub fn new() -> Self
    {
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

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        MyGreeter {
            interface: interface,
            yoffset: yoffset,
            style: style,
            text_style: text_style,
            disp: disp,
        }

    }
}


// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl <C, M, S, SIZE, MODE> Greeter for MyGreeter<C, M, S, SIZE, MODE>
where
C:,
M:,
S:,
SIZE: ,
MODE: ,
{
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Received request from: {:?}", request);

        // Get temp, update screen etc.
        Text::with_baseline(
            &format!("Hello {}!", request.into_inner().name),
            Point::new(20, 40),
            self.text_style,
            Baseline::Top,
        )
        .draw(&mut self.disp)
        .unwrap();

        let response = greeter::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(response))
    }
}


// Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:2424".parse()?;
    let mut greeter = MyGreeter::new();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
