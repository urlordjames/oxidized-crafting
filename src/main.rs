use std::net::{TcpListener, TcpStream};

mod packet;
use packet::{Packet, write_packet};
use packet::packet_data::{read_long, write_long, write_string};

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
			(_, State::Login) => {
				let mut reason_buf = vec![];
				write_string(&mut reason_buf, r#"{
					"text": "TODO: implement login state"
				}"#);

				write_packet(stream, 0x00, reason_buf);
				return;
			},
			(_, State::Play) => {
				let mut reason_buf = vec![];
				write_string(&mut reason_buf, r#"{
					"text": "TODO: implement play state"
				}"#);

				write_packet(stream, 0x17, reason_buf);
				return;
			},
			(id, state) => todo!("TODO: implement packet with id {:x} in state {:?}", id, state)
		}
	}
}
