use std::io::Result;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const PORT: u16 = 5353;

pub fn advertise() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", PORT))?;
    socket.join_multicast_v4(&MULTICAST_ADDR, &Ipv4Addr::UNSPECIFIED)?;

    println!("Listening on {}:{}", MULTICAST_ADDR, PORT);

    let mut buffer = [0u8; 1024];

    loop {
        let (size, sender) = socket.recv_from(&mut buffer)?;
        let message = &buffer[..size];

        if message.starts_with(b"HELLO?") {
            println!("sender: {:#?}", sender);
        }
    }
}

pub fn find_peer() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_multicast_loop_v4(true)?;
    socket.send_to(b"HELLO? name=Nur", SocketAddrV4::new(MULTICAST_ADDR, PORT))?;
    Ok(())
}
