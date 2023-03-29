use std::{
    io::{ErrorKind, Read, Write},
    time::Duration,
};

use serialport::SerialPort;

fn main() {
    let mut port = tunnel_test::get_port_builder("TUNNEL_Receiver")
        .timeout(Duration::from_millis(100))
        .open_native()
        .unwrap();

    // let mut port = std::fs::File::open("/dev/ttyACM1").unwrap();

    loop {
        let mut buf = [0; 255];
        let count = buf.len();
        port.write(&"f".as_bytes()).unwrap();
        match port.read(&mut buf) {
            Ok(count) => println!("read {}: {}", count, unsafe {
                core::str::from_utf8_unchecked(&buf[..count])
            }),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {}
                _ => panic!("{}\n", e),
            },
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
