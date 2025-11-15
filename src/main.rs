use std::io;
use tun_tap::*;
use etherparse::{Ipv4HeaderSlice, TcpHeader};
use std::collections::HashMap;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)] // I explained in my notes
struct Quad { // Its structure of each Connection.
    // Ipv4,Port
    src: (Ipv4Addr,u16), // I used Ipv4Addr function because the library gives that type
    dst: (Ipv4Addr,u16)  // And its easy to keep understand, Keeps octets [u8; 4].
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::State> = HashMap::new();
    
    let a = 5;
    let b = 4;
    

    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?; //we created normal virtual network instance

    let mut buf = [0u8; 1504]; // window size

    loop {
        let bytes = nic.recv(&mut buf[..])?; // the func needed with mut buf |--| pub fn  recv(&self, buf: &mut [u8]) -> Result<usize> |--|

        let flags = i16::from_be_bytes([buf[0],buf[1]]); // Takes the first 2 bytes in bytes array and converts to integer ( decimal )
        let eth_proto = i16::from_be_bytes([buf[2],buf[3]]); // Takes the second 2 bytes in bytes array and converts to integer ( decimal )
        if eth_proto != 0x800 {
            // not ipv4
            continue;
        }

        // IP LAYER
        match Ipv4HeaderSlice::from_slice(&buf[4..bytes]) {
            Ok(p) => {
                let p_len = p.slice().len(); // protocol len
               
                // At the first, I used payload_len()
                // I misunderstood to get length of protocol header size why its have Result type ? 
                // because this function returns the payload size with checking max header size of ipv4 datagram
                // so i changed with p.slice().len() its returns the real header size

                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                let ttl = p.ttl();
                
                if proto.0 != 0x06 {
                    // not tcp
                    continue;
                }

                match TcpHeader::from_slice(&buf[4 + p_len..]) { //Skipping the header of IPV4 Datagram Header
                    Ok(h) => {
                        connections.entry(Quad { src: (src,h.0.source_port), dst: (dst,h.0.destination_port) })
                        println!("{} -> {}, {}b of tcp to port {}",src,dst,h.0.header_len(),h.0.destination_port)
                    }
                    Err(e) => {
                        println!("Ignoring weird tcp packet: {:?}",e)
                    }
                }
                
            },
            Err(e) => {
                println!("Ignoring weird packet: {:?}",e)
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

    Ok(())
}
