use std::io;

enum State {
	SynRcvd,
	Estab,
	FinWait1,
	FinWait2,
	Closing,
}

impl State {
	fn is_synchronized(&self) -> bool {
		match *self {
			State::SynRcvd => false,
			State::Estab | State::FinWait1 | State::FinWait2 | State::Closing => true,
		}
	}
}

pub struct Connection {
	state: State,
	send: SendSequenceSpace,
	recv: RecvSequenceSpace,
	ip: etherparse::Ipv4Header,
	tcp: etherparse::TcpHeader,
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
		let wnd = 10;
		let mut c = Connection {
			state: State::SynRcvd,
			send: SendSequenceSpace {
				iss: iss,
				una: iss,
				nxt: iss,
				wnd: wnd,
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
			tcp: etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), iss, wnd),
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

		c.tcp.syn = true;
		c.tcp.ack = true;
		c.write(nic, &[])?;
		Ok(Some(c))
	}

	pub fn write<'a>(&mut self, nic: &mut tun_tap::Iface, payload: &[u8]) -> io::Result<usize> {
		use std::io::Write;
		let mut buf = [0u8; 1500];
		self.tcp.sequence_number = self.send.nxt;
		self.tcp.acknowledgment_number = self.recv.nxt;

		let size = std::cmp::min(
			buf.len(),
			self.tcp.header_len() as usize + self.ip.header_len() as usize + payload.len(),
		);
		self.ip.set_payload_len(size);
		let mut unwritten = &mut buf[..];
		self.ip.write(&mut unwritten);
		self.tcp.write(&mut unwritten);
		let payload_bytes = unwritten.write(payload)?;
		let unwritten = unwritten.len();
		self.send.nxt = self.send.nxt.wrapping_add(payload_bytes as u32);
		if self.tcp.syn {
			self.send.nxt = self.send.nxt.wrapping_add(1);
			self.tcp.syn = false;
		}
		if self.tcp.fin {
			self.send.nxt = self.send.nxt.wrapping_add(1);
			self.tcp.fin = false;
		}
		nic.send(&buf[..buf.len() - unwritten])?;
		Ok(payload_bytes)
	}

	pub fn send_rst(&mut self, nic: &mut tun_tap::Iface) -> io::Result<()> {
		self.tcp.rst = true;
		self.tcp.sequence_number = 0;
		self.tcp.acknowledgment_number = 0;
		self.write(nic, &[])?;
		Ok(())
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
		if !is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
			if !self.state.is_synchronized() {
				self.send_rst(nic);
			}
			return Ok(());
		}
		self.send.una = ackn;
		// RCV.NXT =< SEG.SEQ < RCV.NXT + RCV.WND
		let seqn = tcph.sequence_number();
		let mut slen = data.len() as u32;
		if tcph.fin() {
			slen += 1;
		}
		if tcph.syn() {
			slen += 1;
		}
		let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
		if slen == 0 {
			if self.recv.wnd == 0 {
				if seqn != self.recv.nxt {
					return Ok(());
				}
			} else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend) {
				return Ok(());
			}
		} else {
			if self.recv.wnd == 0 {
				return Ok(());
			} else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend)
				&& !is_between_wrapped(
					self.recv.nxt.wrapping_sub(1),
					seqn.wrapping_add(slen - 1),
					wend,
				) {
				return Ok(());
			}
		}
		self.recv.nxt = seqn.wrapping_add(slen);

		match self.state {
			State::SynRcvd => {
				// expect to get an ACK for our SYN
				if !tcph.ack() {
					return Ok(());
				}
				self.state = State::Estab;
				self.tcp.fin = true;
				self.write(nic, &[])?;
				self.state = State::FinWait1;
			}
			State::Estab => {
				unimplemented!();
			}
			State::FinWait1 => {
				if !tcph.fin() || !data.is_empty() {
					unimplemented!();
				}
				self.state = State::FinWait2;
			}
			State::FinWait2 => {
				if !tcph.fin() || !data.is_empty() {
					unimplemented!();
				}
				self.state = State::FinWait2;
			}
			State::Closing => {
				if !tcph.fin() || !data.is_empty() {
					unimplemented!();
				}

				self.tcp.fin = false;
				self.write(nic, &[])?;
				self.state = State::Closing;
			}
		};
		Ok(())
	}
}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
	use std::cmp::Ordering;
	match start.cmp(&x) {
		Ordering::Equal => return false,
		Ordering::Less => {
			if end >= start && end <= x {
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
