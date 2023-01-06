use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RegistryCodec {
	#[serde(rename = "minecraft:dimension_type")]
	dimension_type: Dimensions
}

impl std::default::Default for RegistryCodec {
	fn default() -> Self {
		Self {
			dimension_type: Dimensions::default()
		}
	}
}

#[derive(Serialize, Deserialize)]
struct Dimensions {
	#[serde(rename = "type")]
	t: String,
	value: Vec<Dimension>
}

impl std::default::Default for Dimensions {
	fn default() -> Self {
		Dimensions {
			t: String::from("minecraft:dimension_type"),
			value: vec![]
		}
	}
}

#[derive(Serialize, Deserialize)]
struct Dimension {
	name: String,
	id: i32,
	element: DimensionType
}

#[derive(Serialize, Deserialize)]
struct DimensionType {
	piglin_safe: bool,
	has_raids: bool,
	monster_spawn_light_level: i32,
	monster_spawn_block_light_limit: i32,
	natural: bool,
	ambient_light: f32,
	fixed_time: Option<i64>,
	infiniburn: String,
	respawn_anchor_works: bool,
	has_skylight: bool,
	bed_works: bool,
	effects: String,
	min_y: i32,
	height: i32,
	logical_height: i32,
	coordinate_scale: f64,
	ultrawarm: bool,
	has_ceiling: bool
}