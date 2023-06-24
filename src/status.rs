use crate::packet::write_packet;
use crate::packet::packet_data::write_string;
use crate::text::Text;
use crate::{GAME_VERSION, PROTOCOL_VERSION};

use tokio::io::AsyncWriteExt;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Version {
	name: String,
	protocol: u64
}

#[derive(Debug, Serialize)]
struct Player {
	name: String,
	id: String
}

#[derive(Debug, Serialize)]
struct Players {
	max: u64,
	online: u64,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	sample: Vec<Player>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
	version: Version,
	players: Players,
	description: Text,
	#[serde(skip_serializing_if = "Option::is_none")]
	favicon: Option<String>,
	previews_chat: bool,
	enforces_secure_chat: bool
}

impl std::default::Default for StatusResponse {
	fn default() -> Self {
		Self {
			version: Version {
				name: String::from(GAME_VERSION),
				protocol: PROTOCOL_VERSION
			},
			players: Players {
				max: 20,
				online: 0,
				sample: vec![]
			},
			description: Text::from("oxidized crafting"),
			favicon: None,
			previews_chat: false,
			enforces_secure_chat: false
		}
	}
}

impl StatusResponse {
	pub async fn write<B: AsyncWriteExt + Unpin>(&self, buffer: &mut B) {
		let stringified = serde_json::to_string(self).unwrap();
		println!("{stringified}");

		let mut packet_data = vec![];
		write_string(&mut packet_data, &stringified).await;

		write_packet(buffer, 0x00, packet_data).await;
	}
}

#[tokio::test]
async fn test_status() {
	use crate::packet::Packet;
	use crate::packet::packet_data::read_string;

	let status = StatusResponse::default();
	let mut packet_buf = vec![];
	status.write(&mut packet_buf).await;

	let mut cursor = std::io::Cursor::new(packet_buf);
	let mut packet = Packet::read(&mut cursor).await;

	let status_json = serde_json::to_string(&status).unwrap();

	assert_eq!(0x00, packet.id);
	assert_eq!(status_json, read_string(&mut packet.data).await);
}
