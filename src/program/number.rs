use super::ProcSymbolError;


pub fn parse(number: &str) -> Result<i32, ProcSymbolError>  {
	if number == "0" {
		return Ok(0);
	}
	if number.starts_with('-') {
		return Ok(-parse(&number[1..])?);
	}
	if number.starts_with('+') {
		return Ok(parse(&number[1..])?);
	}
	if number.starts_with('0') {
		let radix = match number.chars().nth(1).unwrap() {
			'x' | 'X' => 16,
			'd' | 'D' => 10,
			'o' | 'O' => 8,
			'b' | 'B' => 2,
			_ => return Err(ProcSymbolError::BadRadix),
		};
		return match i32::from_str_radix(&number[2..], radix) {
			Ok(v) => Ok(v),
			Err(_) => Err(ProcSymbolError::BadValue),
		};
	}
	return match i32::from_str_radix(number, 10) {
		Ok(v) => Ok(v),
		Err(_) => Err(ProcSymbolError::BadValue),
	};
}

