use crate::packet::Packet;
use crate::packet::packet_data::{read_varint, read_string, read_short};
use crate::{State, LoginState};

#[derive(Debug)]
pub struct Handshake {
	pub protocol_version: u64,
	pub address: String,
	pub port: u16,
	pub next_state: State
}

impl Handshake {
	pub async fn read(packet: &mut Packet) -> Self {
		assert_eq!(packet.id, 0x00);

		let protocol_version = read_varint(&mut packet.data).await;

		assert_eq!(protocol_version, 761);

		let address = read_string(&mut packet.data).await;
		let port = read_short(&mut packet.data).await;
		let next_state = match read_varint(&mut packet.data).await {
			1 => State::Status,
			2 => State::Login(LoginState::PostHandshake),
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
