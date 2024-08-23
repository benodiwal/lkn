use etherparse::{ip_number::TCP, Ipv4HeaderSlice};
use nic::create_nic;

mod nic;
pub mod error;

fn main() -> Result<(), error::Error> {
    let nic = create_nic()?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf)?;
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x8000 {
            continue;
        }

        match Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src_addr = p.source_addr();
                let dest_addr = p.destination_addr();
                let proto = p.protocol();
                if proto != TCP {
                    continue;
                }
                eprintln!("{} -> {} {}b protocol {:?}", src_addr, dest_addr, p.payload_len().unwrap(), proto);
            },
            Err(err) => {
                eprintln!("Ignoring weird packets: {}", err);
            }
        }
    }
}
