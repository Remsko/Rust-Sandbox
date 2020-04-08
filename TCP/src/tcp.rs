use std::io;

enum State {
	SynRcvd,
	Estab,
}

pub struct Connection {
	state: State,
	send: SendSequenceSpace,
	recv: RecvSequenceSpace,
	ip: etherparse::Ipv4Header,
}

struct SendSequenceSpace {
	// send unacknowledged
	una: u32,
	// send next
	nxt: u32,
	// send window
	wnd: u16,
	// send urgent pointer
	up: bool,
	// segment sequence number usded for last window update
	wl1: usize,
	// segment acknowledgement number used for last window  update
	wl2: usize,
	// initial send sequence number
	iss: u32,
}

struct RecvSequenceSpace {
	// receive next
	nxt: u32,
	// receive window
	wnd: u16,
	// receive urgent pointer
	up: bool,
	// initial receive sequence number
	irs: u32,
}

impl Connection {
	pub fn accept<'a>(
		nic: &mut tun_tap::Iface,
		iph: etherparse::Ipv4HeaderSlice<'a>,
		tcph: etherparse::TcpHeaderSlice<'a>,
		data: &'a [u8],
	) -> io::Result<Option<Self>> {
		let mut buf = [0u8; 1500];
		if !tcph.syn() {
			// only expected SYN packet
			return Ok(None);
		}

		let iss = 0;
		let mut c = Connection {
			state: State::SynRcvd,
			send: SendSequenceSpace {
				iss: iss,
				una: iss,
				nxt: iss + 1,
				wnd: 10,
				up: false,

				wl1: 0,
				wl2: 0,
			},
			recv: RecvSequenceSpace {
				nxt: tcph.sequence_number() + 1,
				wnd: tcph.window_size(),
				irs: tcph.sequence_number(),
				up: false,
			},
			ip: etherparse::Ipv4Header::new(
				0,
				64,
				etherparse::IpTrafficClass::Tcp,
				[
					iph.destination()[0],
					iph.destination()[1],
					iph.destination()[2],
					iph.destination()[3],
				],
				[
					iph.source()[0],
					iph.source()[1],
					iph.source()[2],
					iph.source()[3],
				],
			),
		};

		// need to start establishing a connection
		let mut syn_ack = etherparse::TcpHeader::new(
			tcph.destination_port(),
			tcph.source_port(),
			c.send.iss,
			c.send.wnd,
		);
		syn_ack.acknowledgment_number = c.recv.nxt;
		syn_ack.syn = true;
		syn_ack.ack = true;
		c.ip.set_payload_len(syn_ack.header_len() as usize + 0);
		// kernel does it
		// syn_ack.checksum = syn_ack
		// 	.calc_checksum_ipv4(&c.ip, &[])
		// 	.expect("failed to compute checksum");

		// write out the headers
		let unwritten = {
			let mut unwritten = &mut buf[..];
			c.ip.write(&mut unwritten);
			syn_ack.write(&mut unwritten);
			unwritten.len()
		};
		nic.send(&buf[..buf.len() - unwritten])?;
		Ok(Some(c))
	}

	pub fn on_packet<'a>(
		&mut self,
		nic: &mut tun_tap::Iface,
		iph: etherparse::Ipv4HeaderSlice<'a>,
		tcph: etherparse::TcpHeaderSlice<'a>,
		data: &'a [u8],
	) -> io::Result<()> {
		// SND.UNA < SEG.ACK =< SND.NXT
		let ackn = tcph.acknowledgment_number();
		if self.send.una < ackn {
			if self.send.nxt >= self.send.una && self.send.nxt < ackn {
				return Ok(());
			}
		} else {
			if self.send.nxt >= ackn && self.send.nxt < self.send.una {
			} else {
				return Ok(());
			}
		}
		// RCV.NXT =< SEG.SEQ < RCV.NXT + RCV.WND
		match self.state {
			State::SynRcvd => {
				// expect to get an ACK for our SYN
			}
			State::Estab => {
				unimplemented!();
			}
		};
		Ok(())
	}

	fn is_between_wrapped(start: usize, x: usize, end: usize) -> bool {
		use std::cmp::Ordering;
		match start.cmp(x) {
			Ordering::Equal => return false,
			Ordering::Less => {
				if end >= start && end < x {
					return false;
				}
			}
			Ordering::Greater => {
				if end > x && end < start {
				} else {
					return false;
				}
			}
		}
		true
	}
}
