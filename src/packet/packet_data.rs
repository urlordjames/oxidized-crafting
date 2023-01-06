use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TODO: make use of Async*Ext's extra stuff

pub async fn read_varint<P: AsyncReadExt + Unpin>(stream: &mut P) -> u64 {
	const CONTINUE_BIT: u8 = 0x80;
	const SEGMENT_MASK: u8 = 0x7F;

	let mut value: u64 = 0;
	let mut position: usize = 0;
	let mut buf = [0; 1];

	loop {
		stream.read_exact(&mut buf).await.unwrap();

		value |= u64::from(buf[0] & SEGMENT_MASK).to_le() << position;

		if buf[0] & CONTINUE_BIT == 0 {
			break;
		}

		position += 7;

		if position >= 32 {
			panic!("varint too big");
		}
	}

	value
}

pub async fn read_string<P: AsyncReadExt + Unpin>(stream: &mut P) -> String {
	let length = read_varint(stream).await;
	let mut buf = vec![0; length as usize];

	stream.read_exact(&mut buf).await.unwrap();

	String::from_utf8(buf).unwrap()
}

pub async fn read_short<P: AsyncReadExt + Unpin>(stream: &mut P) -> u16 {
	let mut buf = [0; 2];
	stream.read_exact(&mut buf).await.unwrap();

	u16::from_be_bytes(buf)
}

pub async fn read_long<P: AsyncReadExt + Unpin>(stream: &mut P) -> i64 {
	let mut buf = [0; 8];
	stream.read_exact(&mut buf).await.unwrap();

	i64::from_be_bytes(buf)
}

pub async fn read_bool<P: AsyncReadExt + Unpin>(stream: &mut P) -> bool {
	let mut buf = [0; 1];
	stream.read_exact(&mut buf).await.unwrap();

	match buf[0] {
		0x00 => false,
		0x01 => true,
		_ => panic!("invalid bool")
	}
}

pub async fn read_uuid<P: AsyncReadExt + Unpin>(stream: &mut P) -> u128 {
	let mut buf = [0; 16];
	stream.read_exact(&mut buf).await.unwrap();

	u128::from_be_bytes(buf)
}

pub async fn read_int<P: AsyncReadExt + Unpin>(stream: &mut P) -> i32 {
	let mut buf = [0; 4];
	stream.read_exact(&mut buf).await.unwrap();

	i32::from_be_bytes(buf)
}

pub async fn write_varint<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: u64) {
	const CONTINUE_BIT: u64 = 0x80;
	const SEGMENT_MASK: u64 = 0x7F;

	let mut value = value;

	loop {
		if (value & !SEGMENT_MASK) == 0 {
			buffer.write_all(&[value as u8]).await.unwrap();
			return;
		}

		buffer.write_all(&[((value & SEGMENT_MASK) | CONTINUE_BIT) as u8]).await.unwrap();

		value >>= 7;
	}
}

pub async fn write_string<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: &str) {
	let bytes = value.as_bytes();

	write_varint(buffer, bytes.len() as u64).await;
	buffer.write_all(bytes).await.unwrap();
}

pub async fn write_short<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: u16) {
	let bytes = value.to_be_bytes();

	buffer.write_all(&bytes).await.unwrap();
}

pub async fn write_long<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: i64) {
	let bytes = value.to_be_bytes();

	buffer.write_all(&bytes).await.unwrap();
}

pub async fn write_bool<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: bool) {
	buffer.write_all(&[match value {
		false => 0x00,
		true => 0x01
	}]).await.unwrap();
}

pub async fn write_uuid<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: u128) {
	let bytes = value.to_be_bytes();

	buffer.write_all(&bytes).await.unwrap();
}

pub async fn write_int<B: AsyncWriteExt + Unpin>(buffer: &mut B, value: i32) {
	let bytes = value.to_be_bytes();

	buffer.write_all(&bytes).await.unwrap();
}

#[tokio::test]
async fn test_varint() {
	let val = 12345678;
	let mut buf = vec![];

	write_varint(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_varint(&mut cursor).await;

	assert_eq!(val, deserialized);
}

#[tokio::test]
async fn test_string() {
	let val = "hello gamers";
	let mut buf = vec![];

	write_string(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_string(&mut cursor).await;

	assert_eq!(val, deserialized);
}

#[tokio::test]
async fn test_short() {
	let val = u16::MAX;
	let mut buf = vec![];

	write_short(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_short(&mut cursor).await;

	assert_eq!(val, deserialized);
}

#[tokio::test]
async fn test_long() {
	let val = 12345678;
	let mut buf = vec![];

	write_long(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_long(&mut cursor).await;

	assert_eq!(val, deserialized);
}

#[tokio::test]
async fn test_bool() {
	let val = true;
	let mut buf = vec![];

	write_bool(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_bool(&mut cursor).await;

	assert_eq!(val, deserialized);
}

#[tokio::test]
async fn test_uuid() {
	let val = 9876543210123456789;
	let mut buf = vec![];

	write_uuid(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_uuid(&mut cursor).await;

	assert_eq!(val, deserialized);
}

#[tokio::test]
async fn test_int() {
	let val = 12345;
	let mut buf = vec![];

	write_int(&mut buf, val).await;
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_int(&mut cursor).await;

	assert_eq!(val, deserialized);
}
