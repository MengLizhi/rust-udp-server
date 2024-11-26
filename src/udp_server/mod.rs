use std::{net::{SocketAddr, SocketAddrV4, UdpSocket}, sync::mpsc};

struct UdpServer {
    address: SocketAddrV4,
    socket: UdpSocket,
    buf: [u8; 1024],
    receiver: mpsc::Receiver<(usize, SocketAddr)>,
    sender: mpsc::Sender<(usize, SocketAddr)>,
}

impl UdpServer {
    fn new(address: SocketAddrV4) -> Result<Self, String> {
        let res = Self::setup_udp(address);
        return match res {
            Ok(s) => {
                let ( sender, receiver ) = mpsc::channel();
                let server = UdpServer {
                    address,
                    buf: [0; 1024],
                    socket: s,
                    sender,
                    receiver,
                };

                Ok(server)
            }

            Err(e) => Err(e),
        };
    }

    fn setup_udp(address: SocketAddrV4) -> Result<std::net::UdpSocket, String> {
        let socket = UdpSocket::bind(address);
        return match socket {
            Ok(s) => {
                // udp_init(&s);
                return Ok(s);
            }
            Err(e) => {
                let msg = format!("Udp 地址 {:?} 绑定失败, Error: {:?}", address, e);
                Err(msg)
            }
        };
    }

    fn server_init(&self) {

    }
}
