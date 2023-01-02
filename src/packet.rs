use std::io::Read;

pub mod packet_data;
use packet_data::read_varint;

#[derive(Debug)]
pub struct Packet {
	pub id: u64,
	pub data: std::io::Cursor<Vec<u8>>
}

impl Packet {
	pub fn read<P: Read>(stream: &mut P) -> Self {
		let length = read_varint(stream);

		let mut id_and_data = vec![0; length as usize];
		stream.read_exact(&mut id_and_data).unwrap();

		let mut cursor = std::io::Cursor::new(id_and_data);

		let id = read_varint(&mut cursor);

		Self {
			id,
			data: cursor
		}
	}
}
