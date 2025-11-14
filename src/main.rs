use std::io;
use tun_tap::*;
use etherparse::{Ipv4HeaderSlice};

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?; //we created normal virtual network instance

    let mut buf = [0u8; 1504]; // window size

    loop {
        let bytes = nic.recv(&mut buf[..])?; // the func needed with mut buf |--| pub fn  recv(&self, buf: &mut [u8]) -> Result<usize> |--|

        let flags = i16::from_be_bytes([buf[0],buf[1]]); // Takes the first 2 bytes in bytes array and converts to integer ( decimal )
        let eth_proto = i16::from_be_bytes([buf[2],buf[3]]); // Takes the second 2 bytes in bytes array and converts to integer ( decimal )
        if eth_proto != 0x800 {
            // no ipv4
            continue;
        }
        println!("flags {:x} proto {:x}",flags,eth_proto);

        // IP LAYER
        match Ipv4HeaderSlice::from_slice(&buf[4..bytes]) {
            Ok(p) => {
                let p_len = p.payload_len();

                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                let ttl = p.ttl();
                
                if proto.0 != 0x06 {
                    // not tcp
                    continue;
                }

                println!("{} -> {} | proto: {}, hex: {} | ttl: {} | p_len: {}",src,dst,proto.protocol_str().unwrap_or("proto not found"),proto.0,ttl,p_len.unwrap());
            },
            Err(e) => {
                println!("there is a weird packet: {:?}",e)
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
