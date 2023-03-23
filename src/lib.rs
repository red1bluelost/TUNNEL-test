use serialport::{SerialPortBuilder, SerialPortType, UsbPortInfo};

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
