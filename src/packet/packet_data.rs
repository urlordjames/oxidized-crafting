use std::io::{Read, Write};

pub fn read_varint<P: Read>(stream: &mut P) -> u64 {
	const CONTINUE_BIT: u8 = 0x80;
	const SEGMENT_MASK: u8 = 0x7F;

	let mut value: u64 = 0;
	let mut position: usize = 0;
	let mut buf = [0; 1];

	loop {
		stream.read_exact(&mut buf).unwrap();

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

pub fn read_string<P: Read>(stream: &mut P) -> String {
	let length = read_varint(stream);
	let mut buf = vec![0; length as usize];

	stream.read_exact(&mut buf).unwrap();

	String::from_utf8(buf).unwrap()
}

pub fn read_short<P: Read>(stream: &mut P) -> u16 {
	let mut buf = [0; 2];
	stream.read_exact(&mut buf).unwrap();

	u16::from_be_bytes(buf)
}

pub fn read_long<P: Read>(stream: &mut P) -> i64 {
	let mut buf = [0; 8];
	stream.read_exact(&mut buf).unwrap();

	i64::from_be_bytes(buf)
}

pub fn write_varint<B: Write>(buffer: &mut B, value: u64) {
	const CONTINUE_BIT: u64 = 0x80;
	const SEGMENT_MASK: u64 = 0x7F;

	let mut value = value;

	loop {
		if (value & !SEGMENT_MASK) == 0 {
			buffer.write_all(&[value as u8]).unwrap();
			return;
		}

		buffer.write_all(&[((value & SEGMENT_MASK) | CONTINUE_BIT) as u8]).unwrap();

		value >>= 7;
	}
}

pub fn write_string<B: Write>(buffer: &mut B, value: &str) {
	let bytes = value.as_bytes();

	write_varint(buffer, bytes.len() as u64);
	buffer.write_all(bytes).unwrap();
}

pub fn write_short<B: Write>(buffer: &mut B, value: u16) {
	let bytes = value.to_be_bytes();

	buffer.write_all(&bytes).unwrap();
}

pub fn write_long<B: Write>(buffer: &mut B, value: i64) {
	let bytes = value.to_be_bytes();

	buffer.write_all(&bytes).unwrap();
}

#[test]
fn test_varint() {
	let val = 12345678;
	let mut buf = vec![];

	write_varint(&mut buf, val);
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_varint(&mut cursor);

	assert_eq!(val, deserialized);
}

#[test]
fn test_string() {
	let val = "hello gamers";
	let mut buf = vec![];

	write_string(&mut buf, val);
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_string(&mut cursor);

	assert_eq!(val, deserialized);
}

#[test]
fn test_short() {
	let val = u16::MAX;
	let mut buf = vec![];

	write_short(&mut buf, val);
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_short(&mut cursor);

	assert_eq!(val, deserialized);
}

#[test]
fn test_long() {
	let val = 12345678;
	let mut buf = vec![];

	write_long(&mut buf, val);
	let mut cursor = std::io::Cursor::new(buf);
	let deserialized = read_long(&mut cursor);

	assert_eq!(val, deserialized);
}