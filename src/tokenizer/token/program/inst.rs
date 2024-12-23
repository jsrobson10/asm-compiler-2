
pub fn parse_instruction(name: &str) -> Option<(i32, usize)> {
	return Some(match name {
		"stop" => (0x0, 0),
		"set" => (0x1, 2),
		"set_s" => (0x2, 2),
		"copy" => (0x3, 2),
		"swap" => (0x4, 2),
		"store" => (0x5, 2),
		"load" => (0x6, 2),
		"copy_ls" => (0x7, 2),
		"math" => (0x8, 3),
		"jump" => (0x9, 1),
		"jump_if" => (0xa, 2),
		"jump_z" => (0xb, 2),
		"jump_b" => (0xc, 3),
		"call" => (0xd, 3),
		"ret" => (0xe, 1),
		"nop" => (0xf, 0),
		_ => return None,
	});
}

