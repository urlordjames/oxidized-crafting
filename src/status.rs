use crate::packet::packet_data::write_string;

use std::io::Write;
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
	sample: Vec<Player>
}

// maybe move this into it's own file if this is used elsewhere?
#[derive(Debug, Serialize)]
struct Text {
	text: String
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
	version: Version,
	players: Players,
	description: Text,
	favicon: String,
	previewsChat: bool,
	enforcesSecureChat: bool
}

impl std::default::Default for StatusResponse {
	fn default() -> Self {
		Self {
			version: Version {
				name: String::from("1.19.3"),
				protocol: 761
			},
			players: Players {
				max: u64::MAX,
				online: 0,
				sample: vec![]
			},
			description: Text {
				text: String::from("oxidized crafting")
			},
			favicon: String::new(),
			previewsChat: false,
			enforcesSecureChat: false
		}
	}
}

impl StatusResponse {
	pub fn write<B: Write>(&self, buffer: &mut B) {
		let stringified = serde_json::to_string(self).unwrap();
		write_string(buffer, &stringified);
	}
}
