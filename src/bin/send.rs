fn main() {
    let mut port = tunnel_test::get_port_builder("TUNNEL_Leader")
        .open()
        .unwrap();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line_len = line.as_bytes().len();
        let write_len = port.write(line.as_bytes()).unwrap();
        assert_eq!(line_len, write_len);
    }
}
