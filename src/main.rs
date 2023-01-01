use std::net::{TcpListener, TcpStream};
use std::io::Read;

mod util;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

	for stream in listener.incoming() {
		handle_client(&mut stream.unwrap());
	}
}

fn handle_client(stream: &mut TcpStream) {
	let mut state = State::Handshake;

	loop {
		let mut packet = Packet::read(stream);
		println!("{:?}", packet);

		match (packet.id, state) {
			(0x00, State::Handshake) => {
				let handshake = Handshake::read(&mut packet);
				println!("{:?}", handshake);

				status_respond(stream);

				state = handshake.next_state;
			},
			(id, state) => todo!("implement packet with id {:x} in state {:?}", id, state)
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum State {
	Handshake,
	Status,
	Login
}

#[derive(Debug)]
struct Handshake {
	protocol_version: u64,
	address: String,
	port: u16,
	next_state: State
}

impl Handshake {
	fn read(packet: &mut Packet) -> Self {
		assert_eq!(packet.id, 0x00);

		let protocol_version = util::read_varint(&mut packet.data);

		assert_eq!(protocol_version, 761);

		let address = util::read_string(&mut packet.data);
		let port = util::read_short(&mut packet.data);
		let next_state = match util::read_varint(&mut packet.data) {
			1 => State::Status,
			2 => State::Login,
			_ => panic!("invalid next_state")
		};

		Self {
			protocol_version,
			address,
			port,
			next_state
		}
	}
}

#[derive(Debug)]
struct Packet {
	id: u64,
	data: std::io::Cursor<Vec<u8>>
}

impl Packet {
	fn read<P: Read>(stream: &mut P) -> Self {
		let length = util::read_varint(stream);

		let mut id_and_data = vec![0; length as usize];
		stream.read_exact(&mut id_and_data).unwrap();

		let mut cursor = std::io::Cursor::new(id_and_data);

		let id = util::read_varint(&mut cursor);

		Self {
			id,
			data: cursor
		}
	}
}

fn status_respond(stream: &mut TcpStream) {
	todo!("respond to ping")
}
