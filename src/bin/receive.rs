use serialport::{SerialPortType, UsbPortInfo};

fn main() {
    let tunnel_device = "TUNNEL_Follower";
    let path = serialport::available_ports()
        .expect("No ports found!")
        .into_iter()
        .find(|p| match &p.port_type {
            SerialPortType::UsbPort(UsbPortInfo {
                serial_number: Some(sn),
                product: Some(p),
                ..
            }) => sn == "deadbeef" && p == tunnel_device,
            _ => false,
        })
        .unwrap()
        .port_name;

    let mut port = serialport::new(path, 115200).open().unwrap();

    loop {
        if port.bytes_to_read().unwrap() == 0 {
            continue;
        }
        let mut buf = String::new();
        match port.read_to_string(&mut buf) {
            Ok(0) => {}
            Ok(count) => println!("read {}: {}", count, buf),
            Err(e) => panic!("{}", e),
        }
    }
}
