use std::collections::HashMap;

use crate::{parser::Parser, token::Token};

#[derive(Debug,Clone)]
pub struct Subroutine<'a> {
	symbols: HashMap<&'a str, i32>,
	tokens: Vec<Token<'a>>,
	total_size: i32,
	stack_at: i32,
	name: &'a str,
	
	pub program_offset: i32,
}

impl<'a> Subroutine<'a> {
	pub fn new(name: &str) -> Subroutine {
		Subroutine {
			symbols: HashMap::new(),
			tokens: Vec::new(),
			stack_at: 2,
			total_size: 0,
			program_offset: 0,
			name,
		}
	}
	pub fn get_name(&self) -> &'a str {
		return &self.name;
	}
	pub fn get_symbol(&self, name: &str) -> Option<i32> {
		if name.starts_with('-') {
			return Some(-self.get_symbol(&name[1..])?);
		}
		if let Some(v) = self.symbols.get(name) {
			return Some(*v);
		}
		return None;
	}
	pub fn add_token(&mut self, token: Token<'a>) -> Result<(), String> {
		match token {
			Token::Local(name, size) => {
				self.symbols.insert(name, self.stack_at + 0x380);
				self.stack_at += size;
			}
			Token::LocalSet(name, value) => {
				let addr = self.stack_at + 0x380;
				let t = Token::Set(addr, value);
				self.total_size += t.size();
				self.tokens.push(t);
				self.symbols.insert(name, addr);
				self.stack_at += 1;
			}
			Token::Label(name) => {
				self.symbols.insert(name, self.total_size);
			}
			_ => {
				self.total_size += token.size();
				self.tokens.push(token);
			}
		};
		return Ok(());
	}
	pub fn size(&self) -> i32 {
		return self.total_size;
	}
	pub fn proc_symbol(&self, parser: &mut Parser, name: &str) -> Result<i32, String> {
		if let Some(v) = self.get_symbol(name) {
			return Ok(v);
		}
		return parser.proc_symbol(name);
	}
	fn try_get_symbol(&self, parser: &Parser, name: &str) -> Result<i32, String> {
		if let Some(v) = self.get_symbol(name) {
			return Ok(v);
		}
		return parser.get_symbol(name);
	}
	pub fn process_binary(&self, parser: &Parser, binary: &mut Vec<i32>) -> Result<(), String> {
		let mut push_at = 0;
		for token in self.tokens.iter() {
			match token {
				Token::Stop() => {
					binary.extend_from_slice(&[0x000]);
				}
				Token::Set(dst, v) => {
					binary.extend_from_slice(&[0x100, *dst, *v]);
				}
				Token::SetStore(dst, v) => {
					binary.extend_from_slice(&[0x200, *dst, *v]);
				}
				Token::Copy(src, dst) => {
					binary.extend_from_slice(&[0x300, *src, *dst]);
				}
				Token::Swap(a, b) => {
					binary.extend_from_slice(&[0x400, *a, *b]);
				}
				Token::Store(src, dst) => {
					binary.extend_from_slice(&[0x500, *src, *dst]);
				}
				Token::Load(src, dst) => {
					binary.extend_from_slice(&[0x600, *src, *dst]);
				}
				Token::Math(mode, a, b, dst) => {
					binary.extend_from_slice(&[0x700 | *mode, *a, *b, *dst]);
				}
				Token::Jump(name) => {
					binary.extend_from_slice(&[0x800, self.try_get_symbol(parser, name)? + self.program_offset]);
				}
				Token::JumpIf(check, name) => {
					binary.extend_from_slice(&[0x900, *check, self.try_get_symbol(parser, name)? + self.program_offset]);
				}
				Token::JumpIfNot(check, name) => {
					binary.extend_from_slice(&[0xa00, *check, self.try_get_symbol(parser, name)? + self.program_offset]);
				}
				Token::Push(src) => { // copy to just outside the next stack frame
					binary.extend_from_slice(&[0x400, *src, 0x380 + self.stack_at + push_at + 2]);
					push_at += 1;
				}
				Token::Call(name, dst) => { // create next stack frame and call to the subroutine
					binary.extend_from_slice(&[
						0xb00, self.stack_at, 0xd00, parser.get_symbol(&name)?, *dst,
					]);
				}
				Token::Ret(src) => {
					binary.extend_from_slice(&[0xe00, *src]);
					push_at = 0;
				}
				Token::Nop() => {
					binary.extend_from_slice(&[0xf00]);
				}
				_ => {}
			}
		}
		
		return Ok(());
	}
}

