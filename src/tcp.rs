use std::{io, net::Ipv4Addr};

use etherparse::{Ipv4Header, Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice, ip_number::TCP};
use tun_tap::Iface;

pub enum State {
    Closed,
    Listen,
    SynRcvd,
    SynSent,
    Estab
}

impl Default for State {
    fn default() -> Self {
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(&mut self,nic: &mut Iface,iph: Ipv4HeaderSlice<'a>, tcph: TcpHeaderSlice, data: &'a [u8]) -> io::Result<usize>
    // Lifetime Annotation 'a 
    // The datas which have 'a annotation, means they are same lifetime for avoid dangling pointer and data corruption
    {
        let mut unwritten_buffer = [0u8; 1500]; // TO UNDERSTAND

        match self {
            State::Closed => return Ok(0),
            State::Listen => {
                if !tcph.syn() {
                    //only excepted syn packet
                    return Ok(0)
                }
                use std::io::Write;
                
                //need to start establish a connection
                let mut tcp_packet = TcpHeader::new(tcph.source_port(), tcph.destination_port(), 0, 0);
                tcp_packet.ack = true;
                tcp_packet.syn = true;
                if let Ok(mut ipv4_header) = etherparse::Ipv4Header::new(tcp_packet.header_len_u16(), 64, TCP, iph.destination(),iph.source()) {
                    let mut unwritten = &mut unwritten_buffer[..]; // TO UNDERSTAND
                    
                    ipv4_header.write(&mut unwritten); // TO UNDERSTAND
                }
                
                 

                
            },
            State::SynRcvd => todo!(),
            State::SynSent => todo!(),
            State::Estab => todo!(),
        }
        println!(
            "{}:{} -> {}:{}, {}b of tcp",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len()
        );
        return Ok(0)
    }
}
