// Will create an exporter with a single metric that will randomize the value
// of the metric everytime the exporter duration times out.

use env_logger::{Builder, Env};
use log::info;
use prometheus_exporter::prometheus::register_gauge;

use std::net::SocketAddr;

use linux_embedded_hal as hal;

use hal::{Delay, I2cdev};

use bme280::BME280;

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

fn main() {
    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse address used to bind exporter to.
    let addr_raw = "0.0.0.0:9186";
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter and update metrics every five seconds.
    let exporter = prometheus_exporter::start(addr).expect("can not start exporter");
    let duration = std::time::Duration::from_millis(15000);

    // Create metric
    let gtemp = register_gauge!("temp", "Temperature value").expect("can not create gauge temp");
    let gpres = register_gauge!("pres", "Pressure value").expect("can not create gauge pres");
    let ghumi = register_gauge!("humi", "Rel. Humidity value").expect("can not create gauge humi");

    loop {
        {
            // Will block until duration is elapsed.
            let _guard = exporter.wait_duration(duration);

            info!("Updating metrics");

            let data = get_data();
            /*
            let temp = data.temp;
            let pres = data.pres;
            let humi = data.humi;*/
            gtemp.set(data.temp.into());
            gpres.set(data.pres.into());
            ghumi.set(data.humi.into());
        }

        // Get metrics from exporter
        let body = reqwest::blocking::get(&format!("http://{}/metrics", addr_raw))
            .expect("can not get metrics from exporter")
            .text()
            .expect("can not body text from request");

        info!("Exporter metrics:\n{}", body);
    }
}
