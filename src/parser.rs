use std::{collections::HashMap, mem::swap};

use crate::subroutine::Subroutine;

#[derive(Debug)]
pub struct Parser<'a> {
	symbols: HashMap<&'a str, i32>,
	subroutines: Vec<Subroutine<'a>>,
	inline_constants: HashMap<i32, i32>,
	constants_at: i32,
	globals_at: i32,
	binary: Vec<i32>,
	generated: bool,
}

impl<'a> Parser<'a> {
	pub fn new() -> Parser<'a> {
		Parser {
			symbols: HashMap::new(),
			subroutines: Vec::new(),
			inline_constants: HashMap::new(),
			binary: vec![0xd00, 0, 0, 0],
			globals_at: 0x200,
			generated: false,
			constants_at: 4,
		}
	}
	pub fn set_symbol(&mut self, name: &'a str, addr: i32) {
		self.symbols.insert(name, addr);
	}
	pub fn add_constant(&mut self, name: &'a str, values: &[i32]) -> i32 {
		let addr = self.constants_at;
		self.constants_at += values.len() as i32;
		self.binary.extend_from_slice(values);
		self.set_symbol(name, addr);
		return addr;
	}
	pub fn add_inline_constant(&mut self, value: i32) -> i32 {
		let addr = self.constants_at;
		self.constants_at += 1;
		self.inline_constants.insert(value, addr);
		self.binary.push(value);
		return addr;
	}
	pub fn add_global(&mut self, name: &'a str, size: i32) -> i32 {
		let addr = self.globals_at;
		self.globals_at += size;
		self.set_symbol(name, addr);
		return addr;
	}
	pub fn proc_symbol(&mut self, symbol: &str, sr: Option<&Subroutine>) -> Result<i32, String> {
		if symbol.starts_with('-') {
			return Ok(-self.proc_symbol(&symbol[1..], sr)?);
		}
		if symbol.starts_with('&') {
			let value = self.proc_symbol(&symbol[1..], sr)?;
			if let Some(addr) = self.inline_constants.get(&value) {
				return Ok(*addr as i32);
			}
			return Ok(self.add_inline_constant(value));
		}
		if let Some(v) = sr {
			if let Some(v) = v.get_symbol(symbol) {
				return Ok(v);
			}
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
		return Err(format!("Symbol '{}' is missing", symbol));
	}
	pub fn add_subroutine(&mut self, subroutine: Subroutine<'a>) {
		self.subroutines.push(subroutine);
	}
	pub fn add_to_binary(&mut self, data: &[i32]) {
		self.binary.extend_from_slice(data);
	}
	pub fn generate_binary(&mut self, start_name: &str) -> Result<Vec<i32>, String> {
		if self.generated {
			return Err(format!("Cannot generate multiple times"));
		}

		let mut binary = Vec::new();
		let mut subroutines = Vec::new();

		swap(&mut self.binary, &mut binary);
		swap(&mut self.subroutines, &mut subroutines);
		self.generated = true;

		let mut program_size = binary.len() as i32;
		
		for subroutine in subroutines.iter_mut() {
			subroutine.program_offset = program_size;
			self.symbols.insert(&subroutine.get_name(), program_size);
			program_size += subroutine.size();
		}

		binary[1] = self.get_symbol(start_name)?;
		self.constants_at = program_size;

		for subroutine in subroutines.iter() {
			subroutine.process_binary(self, &mut binary)?;
		}

		binary.extend_from_slice(&self.binary);

		return Ok(binary);
	}
}

