use std::io::Write;

#[derive(Debug)]
pub struct Position {
	x: i32,
	y: i16,
	z: i32
}

impl Position {
	pub fn write<B: Write>(&self, buffer: &mut B) {
		let val: i64 = ((i64::from(self.x).to_be() & 0x3ffffff) << 38) | ((i64::from(self.z).to_be() & 0x3ffffff) << 12) | (i64::from(self.y).to_be() & 0xfff);

		buffer.write_all(&val.to_be_bytes()).unwrap();
	}
}

#[derive(Debug)]
pub struct Location {
	pub position: Position,
	pub dimension_name: String
}
