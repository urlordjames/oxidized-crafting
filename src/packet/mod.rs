use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod packet_data;
use packet_data::{read_varint, write_varint};

#[derive(Debug)]
pub struct Packet {
	pub id: u64,
	pub data: std::io::Cursor<Vec<u8>>
}

impl Packet {
	pub async fn read<P: AsyncReadExt + Unpin>(stream: &mut P) -> Self {
		let length = read_varint(stream).await;

		let mut id_and_data = vec![0; length as usize];
		stream.read_exact(&mut id_and_data).await.unwrap();

		let mut cursor = std::io::Cursor::new(id_and_data);

		let id = read_varint(&mut cursor).await;

		Self {
			id,
			data: cursor
		}
	}
}

pub async fn write_packet<B: AsyncWriteExt + Unpin>(buffer: &mut B, id: u64, mut data: Vec<u8>) {
	let mut buf = vec![];
	write_varint(&mut buf, id).await;
	buf.append(&mut data);

	let length = buf.len();

	write_varint(buffer, length as u64).await;
	buffer.write_all(&buf).await.unwrap();
}

#[tokio::test]
async fn test_packet() {
	let data = 56789;
	let mut data_buf = vec![];
	write_varint(&mut data_buf, data).await;

	let mut packet_buf = vec![];
	write_packet(&mut packet_buf, 0x1234, data_buf).await;

	let mut cursor = std::io::Cursor::new(packet_buf);
	let mut packet = Packet::read(&mut cursor).await;

	assert_eq!(data, read_varint(&mut packet.data).await);
}
