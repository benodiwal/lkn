use etherparse::{ip_number::TCP, Ipv4HeaderSlice, TcpHeaderSlice};
use nic::create_nic;

mod nic;
mod error;

fn main() -> Result<(), error::Error> {
    let nic = create_nic()?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf)?;
        let ethernet_frame = &buf[4..nbytes];

        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x8000 {
            continue;
        }

        match Ipv4HeaderSlice::from_slice(ethernet_frame) {
            Ok(ipv4_packet) => {
                let src_addr = ipv4_packet.source_addr();
                let dest_addr = ipv4_packet.destination_addr();

                if ipv4_packet.protocol() == TCP {
                    if let Ok(tcp_packet) = TcpHeaderSlice::from_slice(&ethernet_frame[ipv4_packet.slice().len()..]) {
                        let src_port = tcp_packet.source_port();
                        let dest_port = tcp_packet.destination_port();

                        println!("Received TCP packet: {}:{} -> {}:{}", src_addr, src_port, dest_addr, dest_port);

                        let payload_offset = ipv4_packet.slice().len() + tcp_packet.slice().len();
                        let payload = &ethernet_frame[payload_offset..];

                        if !payload.is_empty() {
                            println!("{:?}", payload);
                        }
                    }
                }
            },
            Err(err) => {
                eprintln!("Ignoring weird packets: {}", err);
            }
        }
    }
}
