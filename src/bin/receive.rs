use std::{
    io::{ErrorKind, Read},
    time::Duration,
};

fn main() {
    let mut port = tunnel_test::get_port_builder("TUNNEL_Follower")
        .timeout(Duration::from_secs(1))
        .open()
        .unwrap();

    loop {
        let mut buf = [0; 255];
        match port.read(&mut buf) {
            Ok(count) => println!("read {}: {}", count, unsafe {
                core::str::from_utf8_unchecked(&buf[..count])
            }),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {}
                _ => panic!("{}\n", e),
            },
        }
    }
}
