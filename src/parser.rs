use std::collections::HashMap;

use crate::subroutine::Subroutine;

#[derive(Debug)]
pub struct Parser<'a> {
	symbols: HashMap<&'a str, i32>,
	subroutines: Vec<Subroutine<'a>>,
	inline_constants: HashMap<i32, i32>,
	globals_at: i32,
	binary: Vec<i32>,
}

impl<'a> Parser<'a> {
	pub fn new() -> Parser<'a> {
		Parser {
			symbols: HashMap::new(),
			subroutines: Vec::new(),
			inline_constants: HashMap::new(),
			binary: vec![0xd00, 0, 0, 0],
			globals_at: 0x200,
		}
	}
	pub fn set_symbol(&mut self, name: &'a str, addr: i32) {
		self.symbols.insert(name, addr);
	}
	pub fn add_constant(&mut self, name: &'a str, values: &[i32]) -> i32 {
		let addr = self.binary.len() as i32;
		self.binary.extend_from_slice(values);
		self.set_symbol(name, addr);
		return addr;
	}
	pub fn add_global(&mut self, name: &'a str, size: i32) -> i32 {
		let addr = self.globals_at;
		self.globals_at += size;
		self.set_symbol(name, addr);
		return addr;
	}
	pub fn proc_symbol(&mut self, symbol: &str) -> Result<i32, String> {
		if symbol.starts_with('-') {
			return Ok(-self.proc_symbol(&symbol[1..])?);
		}
		if symbol.starts_with('&') {
			let value = self.proc_symbol(&symbol[1..])?;
			if let Some(addr) = self.inline_constants.get(&value) {
				return Ok(*addr as i32);
			}
			let addr = self.binary.len() as i32;
			self.inline_constants.insert(value, addr);
			self.binary.push(value);
			return Ok(addr);
		}
		return self.get_symbol(symbol);
	}
	pub fn get_symbol(&self, symbol: &str) -> Result<i32, String> {
		if let Some(ch) = symbol.chars().nth(0) {
			if ch == '-' {
				return Ok(-self.get_symbol(&symbol[1..])?);
			}
			if ('1' <= ch && ch <= '9') || (ch == '0' && symbol.len() == 1) {
				return match i32::from_str_radix(symbol, 10) {
					Ok(v) => Ok(v),
					Err(v) => Err(v.to_string()),
				};
			}
			if ch == '0' && symbol.len() >= 2 {
				let radix = match &symbol[1..2] {
					"x" | "X" => 16,
					"b" | "B" => 2,
					"o" | "O" => 8,
					"d" | "D" => 10,
					_ => return Err(format!("Unknown radix '{}' for symbol '{}'", &symbol[1..2], symbol)),
				};
				return match i32::from_str_radix(&symbol[2..], radix) {
					Ok(v) => Ok(v),
					Err(e) => Err(e.to_string()),
				};
			}
		}
		if let Some(v) = self.symbols.get(symbol) {
			return Ok(*v);
		}
		return Err(format!("Unknown symbol '{}'", symbol));
	}
	pub fn add_subroutine(&mut self, subroutine: Subroutine<'a>) {
		self.subroutines.push(subroutine);
	}
	pub fn generate_binary(&self, start_name: &str) -> Result<Vec<i32>, String> {
		let mut subroutines = self.subroutines.clone();
		let mut subroutines_lookup: HashMap<String, &mut Subroutine> = HashMap::new();
		let mut binary = self.binary.clone();
		let program_size = self.binary.len() as i32;
		
		for subroutine in subroutines.iter_mut() {
			subroutine.program_offset = program_size;
			if !subroutines_lookup.insert(subroutine.get_name().to_string(), subroutine).is_none() {
				return Err(format!("Subroutine '{}' has duplicates", subroutine.get_name()));
			}
		}

		binary[1] = match subroutines_lookup.get(start_name) {
			Some(v) => v.program_offset as i32,
			None => return Err(format!("Subroutine '{}' is missing", start_name)),
		};

		for subroutine in self.subroutines.iter() {
			subroutine.process_binary(self, &mut binary)?;
		}

		return Ok(binary);
	}
}

