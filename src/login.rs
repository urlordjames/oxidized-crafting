use crate::packet::{Packet, write_packet};
use crate::packet::packet_data::{read_string, read_bool, read_uuid, write_uuid, write_string, write_varint};

use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct LoginStart {
	pub name: String,
	pub uuid: Option<u128>
}

impl LoginStart {
	pub async fn read(packet: &mut Packet) -> Self {
		assert_eq!(packet.id, 0x00);

		let name = read_string(&mut packet.data).await;

		match read_bool(&mut packet.data).await {
			true => Self {
				name,
				uuid: Some(read_uuid(&mut packet.data).await)
			},
			false => Self {
				name,
				uuid: None
			}
		}
	}
}

pub struct LoginSuccess<'a> {
	pub uuid: u128,
	pub username: &'a str
}

impl<'a> LoginSuccess<'a> {
	pub async fn write<B: AsyncWriteExt + Unpin>(&self, buffer: &mut B) {
		let mut packet_data = vec![];
		write_uuid(&mut packet_data, self.uuid).await;
		write_string(&mut packet_data, self.username).await;
		write_varint(&mut packet_data, 0).await;

		write_packet(buffer, 0x02, packet_data).await;
	}
}
