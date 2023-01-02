use std::net::{TcpListener, TcpStream};

mod packet;
use packet::Packet;

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
			(id, state) => {
				println!("TODO: implement packet with id {:x} in state {:?}", id, state);
				return;
			}
		}
	}
}
