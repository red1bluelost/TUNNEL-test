use std::time::Duration;

use serialport::{SerialPortType, UsbPortInfo};

fn main() {
    let path = serialport::available_ports()
        .expect("No ports found!")
        .into_iter()
        .find(|p| match &p.port_type {
            SerialPortType::UsbPort(UsbPortInfo {
                serial_number: Some(s),
                ..
            }) => s == "deadbeef",
            _ => false,
        })
        .unwrap()
        .port_name;

    let mut port = serialport::new(path, 115200)
        .timeout(Duration::from_secs(10))
        .open()
        .unwrap();

    loop {
        let mut buf = String::new();
        match port.read_to_string(&mut buf) {
            Ok(0) => {}
            Ok(count) => println!("read {}: {}", count, buf),
            Err(e) => panic!("{}", e),
        }
    }
}
