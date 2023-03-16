use serialport::{SerialPortType, UsbPortInfo};

fn main() {
    let tunnel_device = "TUNNEL_Leader";
    let path = serialport::available_ports()
        .expect("No ports found!")
        .into_iter()
        .find(|p| match dbg!(&p.port_type) {
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

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line_len = line.as_bytes().len();
        let write_len = port.write(line.as_bytes()).unwrap();
        assert_eq!(line_len, write_len);
    }
}
