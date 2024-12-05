
pub mod process;
pub mod finalize;

use std::collections::HashMap;

use crate::tokenizer::token::program::ProgramToken;

#[derive(Debug)]
pub struct Subroutine<'a> {
	name: &'a str,
	symbols: HashMap<&'a str, i32>,
	tokens: Vec<ProgramToken<'a>>,
	program_start: i32,
	program_size: i32,
	stack_size: i32,
}

impl<'a> Subroutine<'a> {
	pub fn new(name: &'a str) -> Subroutine {
		Subroutine {
			name,
			symbols: HashMap::new(),
			tokens: Vec::new(),
			program_start: 0,
			program_size: 0,
			stack_size: 2,
		}
	}
	fn add_stack(&mut self, name: &'a str, size: i32) -> i32 {
		let addr = self.stack_size + 0x380;
		self.add_symbol(name, addr);
		self.stack_size += size;
		return addr;
	}
	fn add_symbol(&mut self, name: &'a str, value: i32) {
		self.symbols.insert(name, value);
	}
	fn add_token(&mut self, token: ProgramToken<'a>) {
		self.program_size += token.size() as i32;
		self.tokens.push(token);
	}
	pub fn get_symbol(&self, name: &'a str) -> Option<i32> {
		return match self.symbols.get(name) {
			Some(&v) => Some(v),
			None => None,
		}
	}
	pub fn get_name(&self) -> &'a str {
		return self.name;
	}
	pub fn get_program_size(&self) -> i32 {
		return self.program_size;
	}
}

