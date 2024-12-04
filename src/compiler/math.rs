
pub fn parse_subaction(subaction: &str) -> Option<i32> {
	Some(match subaction {
		"or" => 0x02,
		"nor" => 0x03,
		"and" => 0x04,
		"nand" => 0x05,
		"xor" => 0x06,
		"nxor" => 0x07,
		"add" => 0x08,
		"addc" => 0x09,
		"sub" => 0x0a,
		"subc" => 0x0b,
		"lthan" => 0x10,
		"geq" => 0x11,
		"eq" => 0x12,
		"neq" => 0x13,
		"gthan" => 0x14,
		"leq" => 0x15,
		"lthan_s" => 0x18,
		"geq_s" => 0x19,
		"gthan_s" => 0x1c,
		"leq_s" => 0x1d,
		_ => return None,
	})
}

