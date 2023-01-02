use std::net::{TcpListener, TcpStream};

mod packet;
use packet::{Packet, write_packet};
use packet::packet_data::{read_long, write_long};

mod handshake;
use handshake::Handshake;

mod state;
use state::State;

mod status;
use status::StatusResponse;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

	for stream in listener.incoming() {
		handle_client(&mut stream.unwrap());
	}
}

fn handle_client(stream: &mut TcpStream) {
	println!("new connection");

	let mut state = State::Handshake;

	loop {
		let mut packet = Packet::read(stream);
		println!("{:?}", packet);

		match (packet.id, state) {
			(0x00, State::Handshake) => {
				let handshake = Handshake::read(&mut packet);
				println!("{:?}", handshake);

				state = handshake.next_state;
			},
			(0x00, State::Status) => {
				let resp = StatusResponse::default();
				resp.write(stream);
			},
			(0x01, State::Status) => {
				let ping_data = read_long(&mut packet.data);

				let mut pong_buf = vec![];
				write_long(&mut pong_buf, ping_data);

				write_packet(stream, 0x01, pong_buf);
				return;
			},
			(id, state) => {
				println!("TODO: implement packet with id {:x} in state {:?}", id, state);
				return;
			}
		}
	}
}
