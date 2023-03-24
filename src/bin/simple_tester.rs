use serialport::SerialPort;
use std::{
    io::{self, ErrorKind},
    time::Duration,
};

fn main() -> io::Result<()> {
    let timeout = Duration::from_millis(50);
    let iter_delay = Duration::from_millis(1000);

    let mut leader_port = tunnel_test::get_port_builder("TUNNEL_Leader")
        .timeout(timeout)
        .open()
        .unwrap();

    let mut follower_port = tunnel_test::get_port_builder("TUNNEL_Follower")
        .timeout(timeout)
        .open()
        .unwrap();

    let mut cnt: u64 = 0;
    loop {
        back_and_forth_test(leader_port.as_mut(), follower_port.as_mut())?;
        println!("test iteration {} successful", cnt);
        cnt = cnt.wrapping_add(1);
        std::thread::sleep(iter_delay);
    }
}

fn back_and_forth_test(
    leader_port: &mut dyn SerialPort,
    follower_port: &mut dyn SerialPort,
) -> io::Result<()> {
    let leader_to_follower_msg = "hello follower".as_bytes();
    let write_size = leader_port.write(&leader_to_follower_msg)?;
    assert_eq!(write_size, leader_to_follower_msg.len());

    let read = loop {
        let mut buf = vec![0; 255];
        match follower_port.read(&mut buf) {
            Ok(count) => {
                buf.truncate(count);
                break buf;
            }
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {}
                _ => return Err(e),
            },
        }
    };
    assert_eq!(read, leader_to_follower_msg);

    let follower_to_leader_msg = "hello leader".as_bytes();
    let write_size = follower_port.write(&follower_to_leader_msg)?;
    assert_eq!(write_size, follower_to_leader_msg.len());

    let read = loop {
        let mut buf = vec![0; 255];
        match leader_port.read(&mut buf) {
            Ok(count) => {
                buf.truncate(count);
                break buf;
            }
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {}
                _ => return Err(e),
            },
        }
    };
    assert_eq!(read, follower_to_leader_msg);

    Ok(())
}
