use std::net::Ipv4Addr;

use etherparse::{Ipv4Header, Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice, ip_number::TCP};

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
    pub fn on_packet<'a>(&mut self,iph: Ipv4HeaderSlice<'a>, tcph: TcpHeaderSlice, data: &'a [u8])
    //Lifetime Annotation 'a 
    // The datas which have 'a annotation, means they are same lifetime for avoid dangling pointer and data corruption
    {
        match self {
            State::Closed => return,
            State::Listen => {
                if !tcph.syn() {
                    //only excepted syn packet
                    return
                }

                //need to start establish a connection
                let mut tcp_packet = TcpHeader::new(tcph.source_port(), tcph.destination_port(), 0, 0);
                tcp_packet.ack = true;
                tcp_packet.syn = true;
                let mut ipv4_packet = Ipv4Header::new(tcp_packet.header_len_u16(), 64, TCP, iph.destination(),iph.source());

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
        )
    }
}
