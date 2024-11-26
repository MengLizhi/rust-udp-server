use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use tokio::io;

mod udp_server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 18801);

    let s = setup_udp(address);
    match s {
        Ok(_) => {
            println!("UDP Server setup success, address is {:?}", address.to_string())
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
    Ok(())
}

fn setup_udp(address: SocketAddrV4) -> Result<std::net::UdpSocket, String> {
    let socket = UdpSocket::bind(address);
    return match socket {
        Ok(s) => {
            udp_init(&s);
            return Ok(s);
        }
        Err(e) => {
            let msg = format!("Udp 地址 {:?} 绑定失败, Error: {:?}", address, e);
            Err(msg)
        }
    };
}

fn udp_init(socket: &UdpSocket) {
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                let msg = String::from_utf8_lossy(&buf[..len]);
                println!("从 {:?} Udp 接受消息：{:?}", src, msg);
            }
            Err(e) => {
                println!("Udp 消息解析错误，Error: {:?}", e);
            }
        }
    }
}
