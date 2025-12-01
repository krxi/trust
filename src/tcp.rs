use std::io;

use etherparse::{Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice, ip_number::TCP};
use tun_tap::Iface;

#[allow(dead_code)]
enum State {
    Closed,
    Listen,
    SynRcvd,
    SynSent,
    Estab,
}

pub struct Connection {
    state: State,
}

#[doc = r"State of the Send Sequence Space (rfc793 S3.2 F4)
```
           1         2          3          4
      ----------|----------|----------|----------
             SND.UNA    SND.NXT    SND.UNA
                                  +SND.WND

1 - old sequence numbers which have been acknowledged
2 - sequence numbers of unacknowledged data
3 - sequence numbers allowed for new data transmission
4 - future sequence numbers which are not yet allowed
```"]


struct SendSequence {
    // send unacknowledged
    una: usize,
    // send next
    nxt: usize,
    // send window
    wnd: usize,
    //send urgent point (idk what is it)
    up: bool, 
    // segment sequence number used for last window update
    wl1: usize,
    // segment acknowledgment number used for last window update
    wl2: usize,
    // initial send sequence number
    iss: usize

}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            state: State::Listen,
        }
    }
}

impl Connection {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut Iface,
        iph: Ipv4HeaderSlice<'a>,
        tcph: TcpHeaderSlice,
        data: &'a [u8],
    ) -> io::Result<usize>
// Lifetime Annotation 'a 
    // The datas which have 'a annotation, means they are same lifetime for avoid dangling pointer and data corruption
    {
        let mut unwritten_buffer = [0u8; 1500]; // TO UNDERSTAND
        
        match self.state {
            State::Closed => return Ok(0),
            State::Listen => {
                if !tcph.syn() {
                    //only excepted syn packet
                    return Ok(0);
                }

                //need to start establish a connection
                let mut tcp_packet =
                    TcpHeader::new(tcph.destination_port(), tcph.source_port(), 0, 64);
                    // Out seq number is 0 for now. TO-DO 
                
                tcp_packet.ack = true;
                tcp_packet.syn = true;
                tcp_packet.acknowledgment_number = tcph.sequence_number() + 1; // Because we got SYN, so we need to increase by 1.

                if let Ok(ipv4_header) = etherparse::Ipv4Header::new(
                    tcp_packet.header_len_u16(),
                    64,
                    TCP,
                    iph.destination(),
                    iph.source(),
                ) {
                    let total_len= ipv4_header.header_len() + tcp_packet.header_len();
                    println!("prepared packet total len: {}",total_len);

                    println!("ipv4 total len (test) {}",ipv4_header.total_len);

                    //let mut test = &mut unwritten_buffer;
                    let slice = &mut &mut unwritten_buffer[0..total_len]; // converting slice as mutable.

                    ipv4_header.write(slice)?; // first ipv4
                    tcp_packet.write(slice)?; // second tcp packet
                    
                    nic.send(&unwritten_buffer[0..total_len])?; // and send with slice

                    // debug

                    /*
                    println!("{}",i16::from_be_bytes([unwritten_buffer[0],unwritten_buffer[1]]));
                    println!("{}",i16::from_be_bytes([unwritten_buffer[2],unwritten_buffer[3]]));
                    println!("\n{:?}",unwritten_buffer);
                    */
                }
               
            }
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
        return Ok(0);
    }
}
