use std::io::Read;

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
