
pub fn parse_subaction(subaction: &str) -> Option<i32> {
	Some(match subaction {
		"mul" => 0x01,
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
		"lshift_s" => 0x0c,
		"rshift_s" => 0x0d,
		"lshift" => 0x0e,
		"rshift" => 0x0f,
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
		"div_s" => 0x20,
		"mod_s" => 0x21,
		"div" => 0x22,
		"mod" => 0x23,
		_ => return None,
	})
}

