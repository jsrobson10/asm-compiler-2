use std::io::{stdout, StdoutLock, Write};

const HEX: &[u8] = b"0123456789abcdef";

fn write_hex_number(out: &mut StdoutLock, value: i32, digits: i32) -> Result<(), std::io::Error> {
	for i in (0..digits).rev() {
		let v = (value >> (i * 4)) & 15;
		out.write(&[HEX[v as usize]])?;
	}
	return Ok(());
}

fn write_repeat(out: &mut StdoutLock, byte: u8, count: i32) -> Result<(), std::io::Error>{
	for _ in 0..count {
		out.write(&[byte])?;
	}
	return Ok(());
}

pub fn display_hex(binary: &[i32], size: [i32; 2], digits: i32) -> Result<(), std::io::Error> {
	let mut out = stdout().lock();

	out.write(b"\n  ")?;
	write_repeat(&mut out, b' ', digits)?;

	for x in 0..size[0] {
		out.write(b" ")?;
		write_hex_number(&mut out, x, digits)?;
	}
	out.write(b"\n\n")?;

	for y in 0..size[1] {
		out.write(b" ")?;
		write_hex_number(&mut out, y * size[0], digits)?;
		out.write(b" ")?;
		for x in 0..size[0] {
			out.write(b" ")?;
			let i = y * size[0] + x;
			match binary.get(i as usize) {
				Some(&v) => write_hex_number(&mut out, v, digits),
				None => write_repeat(&mut out, b'.', digits),
			}?;
		}
		out.write(b"\n")?;
	}

	return Ok(());
}

