#![allow(unused)]
use crate::file::utils::write_all_vectored;
use crate::protocol::Hello;
use crate::{Result, protocol::Message};

use lipi::{Decode, Encode};
use std::io::{self, BufRead, BufReader, BufWriter, IoSlice, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

pub struct EndPoint {
    tcp: TcpListener,
}

impl EndPoint {
    pub fn new(port: u16) -> Result<Self> {
        match local_ip_address::local_ip() {
            Ok(local_ip) => Self::listen((local_ip, port)),
            Err(_) => Self::listen((Ipv4Addr::UNSPECIFIED, port)),
        }
    }

    pub fn listen(addr: impl ToSocketAddrs) -> Result<Self> {
        Ok(EndPoint {
            tcp: TcpListener::bind(addr)?,
        })
    }

    pub fn accept(&mut self) -> Result<Conn> {
        let (stream, addr) = self.tcp.accept()?;
        let conn = Conn::new(stream, addr)?;
        Ok(conn)
    }
}

pub struct Conn {
    addr: SocketAddr,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,

    len_buf: [u8; 4],
}

impl Conn {
    fn new(stream: TcpStream, addr: SocketAddr) -> io::Result<Self> {
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);

        Ok(Conn {
            addr,
            reader,
            writer,
            len_buf: [0; 4],
        })
    }

    pub fn open(addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        let addr = stream.peer_addr()?;

        let mut conn = Conn::new(stream, addr)?;

        conn.send_hello()?;

        let response = match conn.read_message()? {
            Message::Handshake(hello) => hello,
            _ => return Err("Expected Hello".into()),
        };

        response.check_protocol_version(10..=19)?;

        Ok(conn)
    }

    pub fn send_frame(&mut self, buf: &[u8]) -> Result<()> {
        let len = u32::try_from(buf.len())?;
        let raw_len = len.to_ne_bytes();

        write_all_vectored(
            &mut self.writer,
            &mut [IoSlice::new(&raw_len), IoSlice::new(buf)],
        )?;

        Ok(())
    }

    pub fn send_hello(&mut self) -> Result<()> {
        let msg = Message::Handshake(Hello::message());
        self.send_frame(&msg.to_bytes()?);
        Ok(())
    }

    pub fn read_frame(&mut self) -> io::Result<Vec<u8>> {
        self.reader.read_exact(&mut self.len_buf)?;
        let len = u32::from_ne_bytes(self.len_buf) as usize;

        let mut buf = vec![0; len];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_message(&mut self) -> Result<Message> {
        let buf = self.read_frame()?;
        Ok(Message::decode(&mut buf.as_slice())?)
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    const PORT: u16 = 54231;

    #[test]
    fn test_endpoint() -> Result<()> {
        let mut server = EndPoint::new(PORT)?;
        let addr = server.tcp.local_addr()?;

        thread::scope(|c| {
            c.spawn(|| {
                let conn = server.accept().unwrap();
                println!("conn: {:#?}", conn.addr);
            });

            c.spawn(|| {
                Conn::open(addr);
            });
        });

        Ok(())
    }
}
