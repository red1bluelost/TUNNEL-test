use serialport::{SerialPortType, UsbPortInfo};
use std::{
    io::{ErrorKind, Read},
    time::Duration,
};

fn main() {
    let tunnel_device = "TUNNEL_Follower";
    let path = serialport::available_ports()
        .expect("No ports found!")
        .into_iter()
        .find(|p| match &dbg!(p).port_type {
            SerialPortType::UsbPort(UsbPortInfo {
                serial_number: Some(sn),
                product: Some(p),
                ..
            }) => sn == "deadbeef" && p == tunnel_device,
            _ => false,
        })
        .unwrap()
        .port_name;

    let mut port = serialport::new(path, 115200)
        .timeout(Duration::from_secs(1))
        .open()
        .unwrap();

    loop {
        let mut buf = [0; 255];
        match port.read(&mut buf) {
            Ok(count) => println!("read {}: {}", count, unsafe {
                core::str::from_utf8_unchecked(&buf)
            }),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {}
                _ => panic!("{}\n", e),
            },
        }
    }
}
