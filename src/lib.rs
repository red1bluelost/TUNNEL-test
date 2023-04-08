use serialport::{SerialPort, SerialPortBuilder, SerialPortType, UsbPortInfo};
use std::io::ErrorKind;

pub fn get_port_builder(device_name: &str) -> SerialPortBuilder {
    let path = serialport::available_ports()
        .expect("No ports found!")
        .into_iter()
        .find(|p| match &dbg!(p).port_type {
            SerialPortType::UsbPort(UsbPortInfo {
                serial_number: Some(sn),
                product: Some(p),
                ..
            }) => sn == "deadbeef" && p == device_name,
            _ => false,
        })
        .unwrap()
        .port_name;

    serialport::new(path, 57600)
}

pub fn flush_receiver(receiver_port: &mut dyn SerialPort) {
    let mut cnt = 0;
    loop {
        let mut buf = vec![0; 255];
        receiver_port.write("f".as_bytes()).unwrap();
        match receiver_port.read(&mut buf) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {
                    if cnt >= 10 {
                        return;
                    }
                    cnt += 1;
                }
                _ => panic!("{}", e),
            },
        }
    }
}
