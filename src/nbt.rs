use std::io::Write;

pub enum Nbt {
	Empty
}

impl Nbt {
	pub fn write<B: Write>(&self, buffer: &mut B) {
		buffer.write(&[0x00]);
	}
}
