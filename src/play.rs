use crate::nbt::Nbt;
use crate::position::Location;
use crate::packet::write_packet;
use crate::packet::packet_data::{write_int, write_bool, write_varint, write_string, write_long};

use std::io::Write;

enum Gamemode {
	Survival,
	Creative,
	Adventure,
	Spectator
}

impl Gamemode {
	fn get_id(&self) -> u8 {
		match self {
			Gamemode::Survival => 0,
			Gamemode::Creative => 1,
			Gamemode::Adventure => 2,
			Gamemode::Spectator => 3
		}
	}

	pub fn write<B: Write>(&self, buffer: &mut B) {
		buffer.write_all(&[self.get_id()]).unwrap();
	}
}

enum OptionalGamemode {
	None,
	Gamemode(Gamemode)
}

impl OptionalGamemode {
	pub fn write<B: Write>(&self, buffer: &mut B) {
		let id: i8 = match self {
			OptionalGamemode::None => -1,
			OptionalGamemode::Gamemode(gamemode) => gamemode.get_id() as i8,
		};

		buffer.write_all(&[id as u8]).unwrap();
	}
}

pub struct Login {
	entity_id: i32,
	is_hardcore: bool,
	gamemode: Gamemode,
	previous_gamemode: OptionalGamemode,
	dimensions: Vec<String>,
	registry_codec: Nbt,
	current_dimension_type: String,
	current_dimension_name: String,
	hashed_seed: i64,
	view_distance: u64,
	simulation_distance: u64,
	reduced_debug_info: bool,
	enable_respawn_screen: bool,
	debug_world: bool,
	is_flat: bool,
	death_location: Option<Location>
}

impl std::default::Default for Login {
	fn default() -> Self {
		Self {
			entity_id: 0,
			is_hardcore: false,
			gamemode: Gamemode::Spectator,
			previous_gamemode: OptionalGamemode::None,
			dimensions: vec![],
			registry_codec: Nbt::Empty, // bruh
			current_dimension_type: String::from("minecraft:overworld"),
			current_dimension_name: String::from("overworld"),
			hashed_seed: 0,
			view_distance: 2,
			simulation_distance: 8,
			reduced_debug_info: false,
			enable_respawn_screen: true,
			debug_world: true,
			is_flat: false,
			death_location: None
		}
	}
}

impl Login {
	pub fn write<B: Write>(&self, buffer: &mut B) {
		let mut packet_data = vec![];
		write_int(&mut packet_data, self.entity_id);
		write_bool(&mut packet_data, self.is_hardcore);
		self.gamemode.write(&mut packet_data);
		self.previous_gamemode.write(&mut packet_data);
		write_varint(&mut packet_data, self.dimensions.len() as u64);
		assert_eq!(self.dimensions.len(), 0); // lets just ignore this for now...
		self.registry_codec.write(&mut packet_data);
		write_string(&mut packet_data, &self.current_dimension_type);
		write_string(&mut packet_data, &self.current_dimension_name);
		write_long(&mut packet_data, self.hashed_seed);
		write_varint(&mut packet_data, 0);
		write_varint(&mut packet_data, self.view_distance);
		write_varint(&mut packet_data, self.simulation_distance);
		write_bool(&mut packet_data, self.reduced_debug_info);
		write_bool(&mut packet_data, self.enable_respawn_screen);
		write_bool(&mut packet_data, self.debug_world);
		write_bool(&mut packet_data, self.is_flat);
		match &self.death_location {
			Some(location) => {
				write_bool(&mut packet_data, true);
				write_string(&mut packet_data, &location.dimension_name);
				location.position.write(&mut packet_data);
			},
			None => write_bool(&mut packet_data, false)
		};

		write_packet(buffer, 0x24, packet_data);
	}
}
