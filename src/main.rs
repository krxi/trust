use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)] // I explained in my notes
struct Quad {
    // Its structure of each Connection.
    // Ipv4,Port
    src: (Ipv4Addr, u16), // I used Ipv4Addr function because the library gives that type
    dst: (Ipv4Addr, u16), // And its easy to keep understand, Keeps octets [u8; 4].
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::Connection> = HashMap::new();

    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?; //we created normal virtual network instance
    
    let mut buf = [0u8; 1504]; // MTU (The maximum transmission )
    // if the payload bigger than 1500 (4 for the tun tap flags), The payload divides into parts which not exceeded to 1500.

    loop {
        let bytes = nic.recv(&mut buf[..])?; // the func needed with mut buf |--| pub fn  recv(&self, buf: &mut [u8]) -> Result<usize> |--|

        let _flags = i16::from_be_bytes([buf[0], buf[1]]); // Takes the first 2 bytes in bytes array and converts to integer ( decimal )
        let eth_proto = i16::from_be_bytes([buf[2], buf[3]]); // Takes the second 2 bytes in bytes array and converts to integer ( decimal )
        if eth_proto != 0x0800 {
            // not ipv4
            continue;
        }
       
        // IP LAYER
        match Ipv4HeaderSlice::from_slice(&buf[4..bytes]) {
            Ok(iph) => {
                // At the first, I used payload_len()
                // I misunderstood to get length of protocol header size why its have Result type ?
                // because this function returns the payload size with checking max header size of ipv4 datagram
                // so i changed with p.slice().len() its returns the real header size

                let p_len = iph.slice().len(); // protocol len
                let src = iph.source_addr();
                let dst = iph.destination_addr();

                if iph.protocol().0 != 0x06 {
                    // not tcp
                    continue;
                }

                match TcpHeaderSlice::from_slice(&buf[4 + p_len..bytes]) {
                    // Skipping the header of IPV4 Datagram Header
                    Ok(tcph) => {
                        let data = 4 + p_len + tcph.slice().len();

                        connections
                            .entry(Quad {
                                src: (src, tcph.source_port()),
                                dst: (dst, tcph.destination_port()),
                            })
                            .or_default()
                            .on_packet(&mut nic,iph, tcph, &buf[data..bytes])?;
                    }
                    Err(e) => {
                        println!("Ignoring weird tcp packet: {:?}", e)
                    }
                }
            }
            Err(e) => {
                println!("Ignoring weird packet: {:?}", e)
            }
        }

        //println!("read {} bytes: {:x?}", bytes - 4, &buf[4..bytes]);
        // and we changes bytes to bytes -4 (because we are deleting flags and proto for real total byte info)
        // and we used the first 4 byte for finding flag and proto so we are starting with fourth byte
        // &buf[..bytes] to &buf[4..bytes]

        // we are writing &buf[..bytes] -> ..bytes because
        // rustlang automatic fill the vector automaticly with
        // zeros and if we try to write every data
        // the program write every 0 and its not efficient
    }

}
