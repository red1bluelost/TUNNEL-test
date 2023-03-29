use rand::Rng;
use serialport::SerialPort;
use std::{
    io::{self, ErrorKind},
    time::Duration,
};

fn main() -> io::Result<()> {
    let timeout = Duration::from_millis(50);
    let iter_delay = Duration::from_millis(50);

    let mut sender_port = tunnel_test::get_port_builder("TUNNEL_Leader")
        .timeout(timeout)
        .open()
        .unwrap();

    let mut receiver_port = tunnel_test::get_port_builder("TUNNEL_Receiver")
        .timeout(timeout)
        .open()
        .unwrap();

    let mut cnt: u64 = 0;
    loop {
        match back_and_forth_test(sender_port.as_mut(), receiver_port.as_mut())
        {
            Ok(size) => {
                println!("test iteration {} successful with size {}", cnt, size)
            }
            Err(msg) => {
                println!("test iteration {} failed with {}", cnt, msg)
            }
        }
        cnt = cnt.wrapping_add(1);
        std::thread::sleep(iter_delay);
    }
}

fn back_and_forth_test(
    sender_port: &mut dyn SerialPort,
    receiver_port: &mut dyn SerialPort,
) -> Result<usize, String> {
    let leader_to_follower_msg = rand_buf();
    let buf_size = leader_to_follower_msg.len();
    let write_size = sender_port.write(&leader_to_follower_msg).unwrap();
    if write_size != buf_size {
        return Err(format!(
            "length mismatch on write: buf is {}, write did {}",
            buf_size, write_size
        ));
    }

    let mut cnt = 0;
    let read = loop {
        let mut buf = vec![0; 255];
        receiver_port.write("f".as_bytes()).unwrap();
        match receiver_port.read(&mut buf) {
            Ok(count) => {
                if count != buf_size {
                    return Err(format!(
                        "length mismatch on read: buf is {}, read is {}",
                        buf_size, count
                    ));
                }
                buf.truncate(count);
                break buf;
            }
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {}
                _ => panic!("{}", e),
            },
        }
        if cnt >= 10 {
            return Err(format!(
                "failed iteration of size {}",
                leader_to_follower_msg.len()
            ));
        }
        cnt += 1;
    };
    if read != leader_to_follower_msg {
        return Err("data mismatch".to_string());
    }
    Ok(read.len())
}

fn rand_buf() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(1..50);
    // let size = rng.gen_range(31..=32);
    let size = if matches!(size, 31 | 32 | 64) {
        size + 2
    } else {
        size
    };
    assert_ne!(size, 31);
    assert_ne!(size, 32);
    assert_ne!(size, 64);
    let mut buf = Vec::with_capacity(size);
    for _ in 0..size {
        buf.push(rng.gen());
    }
    buf
}
