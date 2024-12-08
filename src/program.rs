mod number;
pub mod finalize;

use std::collections::HashMap;

use crate::{error::CompileError, parser::subroutine::Subroutine, tokenizer::text::SourceRef};

pub enum ProcSymbolError {
	NotFound,
	BadValue,
	BadRadix,
}

impl ProcSymbolError {
	pub fn str(&self) -> &'static str {
		match self {
			ProcSymbolError::NotFound => "Symbol not found",
			ProcSymbolError::BadValue => "Bad symbol value",
			ProcSymbolError::BadRadix => "Unknown number radix",
		}
	}
	pub fn to_error<'a>(&self, sref: SourceRef<'a>) -> CompileError<'a> {
		return CompileError::new(sref, self.str().to_string());
	}
}

#[derive(Debug)]
pub struct Program<'a> {
	symbols: HashMap<&'a str, i32>,
	constants: Vec<i32>,
	inline_constants: HashMap<i32, i32>,
	program_end: i32,
	global_at: i32,
}

impl<'a> Program<'a> {
	pub fn new() -> Program<'a> {
		let mut program = Program {
			symbols: HashMap::new(),
			constants: Vec::new(),
			inline_constants: HashMap::new(),
			program_end: 5,
			global_at: 0x200,
		};
		program.symbols.insert("null", 0xfff);
		return program;
	}
	pub fn add_subroutine(&mut self, sr: &Subroutine<'a>) {
		self.add_symbol(&sr.get_name(), self.program_end);
		self.program_end += sr.get_program_size();
	}
	pub fn add_symbol(&mut self, name: &'a str, addr: i32) {
		self.symbols.insert(name, addr);
	}
	pub fn add_hidden_global(&mut self, size: i32) -> i32 {
		let addr = self.global_at;
		self.global_at += size;
		return addr;
	}
	pub fn add_global(&mut self, name: &'a str, size: i32) -> i32 {
		let addr = self.add_hidden_global(size);
		self.add_symbol(name, addr);
		return addr;
	}
	pub fn add_constant(&mut self, name: &'a str, values: &[i32]) -> i32 {
		self.constants.extend(values.iter().rev());
		let addr = 0x200 - self.constants.len() as i32;
		self.add_symbol(name, addr);
		return addr;
	}
	pub fn get_program_end(&self) -> i32 {
		return self.program_end;
	}
	fn proc_inline_constant(&mut self, value: i32) -> i32 {
		if let Some(&addr) = self.inline_constants.get(&value) {
			return addr;
		}
		self.constants.push(value);
		let addr = 0x200 - self.constants.len() as i32;
		self.inline_constants.insert(value, addr);
		return addr;
	}
	pub fn proc_symbol(&mut self, sr: Option<&Subroutine>, name: &'a str) -> Result<i32, ProcSymbolError> {
		if let Some(i) = name.find('+') {
			let value = self.proc_symbol_internal(sr, &name[..i])?;
			let offset = number::parse(&name[i+1..])?;
			return Ok(value + offset);
		}
		return self.proc_symbol_internal(sr, name);
	}
	fn proc_symbol_internal(&mut self, sr: Option<&Subroutine>, name: &'a str) -> Result<i32, ProcSymbolError> {
		if name.len() == 0 {
			return Err(ProcSymbolError::BadValue);
		}
		let ch = name.chars().nth(0).unwrap();
		if ch == '-' {
			return Ok(-self.proc_symbol_internal(sr, &name[1..])?);
		}
		if ch == '&' {
			let addr = self.proc_symbol_internal(sr, &name[1..])?;
			return Ok(self.proc_inline_constant(addr));
		}
		if ch >= '0' && ch <= '9' {
			return Ok(number::parse(name)?);
		}
		if let Some(sr) = sr {
			if let Some(v) = sr.get_symbol(name) {
				return Ok(v);
			}
		}
		if let Some(&v) = self.symbols.get(name) {
			return Ok(v);
		}
		return Err(ProcSymbolError::NotFound);
	}
}

