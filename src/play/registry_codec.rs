use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct RegistryCodec {
	#[serde(rename = "minecraft:dimension_type")]
	dimension_type: Dimensions
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
			value: vec![Dimension::default()]
		}
	}
}

#[derive(Serialize, Deserialize)]
struct Dimension {
	name: String,
	id: i32,
	element: DimensionType
}

impl std::default::Default for Dimension {
	fn default() -> Self {
		Self {
			name: String::from("overworld"),
			id: 1,
			element: DimensionType::default()
		}
	}
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

impl std::default::Default for DimensionType {
	fn default() -> Self {
		Self {
			piglin_safe: true,
			has_raids: false,
			monster_spawn_light_level: 15,
			monster_spawn_block_light_limit: 15,
			natural: false,
			ambient_light: 0.2,
			fixed_time: Some(0),
			infiniburn: String::from('#'),
			respawn_anchor_works: false,
			has_skylight: true,
			bed_works: false,
			effects: String::from("minecraft:overworld"),
			min_y: 0,
			height: 256,
			logical_height: 64,
			coordinate_scale: 1.0,
			ultrawarm: false,
			has_ceiling: false
		}
	}
}
