use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{error::CompileError, program::{ProcSymbolError, Program}, tokenizer::{text::SourceRef, token::{program::{self, ProgramToken}, Token}}};

#[derive(Debug)]
pub struct Subroutine<'a> {
	name: &'a str,
	symbols: HashMap<&'a str, i32>,
	tokens: Vec<ProgramToken<'a>>,
	program_size: i32,
	stack_size: i32,
}

impl<'a> Subroutine<'a> {
	pub fn new(name: &'a str) -> Subroutine {
		Subroutine {
			name,
			symbols: HashMap::new(),
			tokens: Vec::new(),
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

pub fn process<'a>(sref_start: SourceRef<'a>, it: &mut Peekable<Iter<Token<'a>>>, program: &mut Program<'a>, name: &'a str) -> Result<(), CompileError<'a>> {
	let mut sr = Subroutine::new(name);

	loop {
		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};

		if token.name.starts_with('.') {
			return Err(CompileError::new(token.sref, "Section cannot follow subroutine".to_string()));
		}

		if token.name.ends_with(':') {
			break;
		}

		match (token.name, token.subaction, token.args.len()) {
			("local", None, 1) => {
				if token.args[0].starts_with('!') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				sr.add_stack(token.args[0], 1);
			}
			("local", None, 2) => {
				if token.args[0].starts_with('!') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				if token.args[1].starts_with('@') {
					let v = match program.proc_symbol(Some(&sr), &token.args[1][1..]) {
						Ok(v) => v,
						Err(err) => return Err(err.to_error(token.sref)),
					};
					sr.add_symbol(token.args[0], v);
				}
				else {
					sr.add_stack(token.args[0], match program.proc_symbol(Some(&sr), &token.args[1]) {
						Ok(v) => v,
						Err(err) => return Err(err.to_error(token.sref)),
					});
				}
			}
			("label", None, 1) => {
				if token.args[0].starts_with('!') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				sr.add_symbol(token.args[0], sr.program_size);
			}
			_ => {
				let mut pt = match ProgramToken::from(token) {
					Ok(pt) => pt,
					Err(err) => return Err(CompileError::new(token.sref, err)),
				};
				for i in 0..pt.args.len() {
					if let program::Arg::Str(name) = pt.args[i] {
						match program.proc_symbol(Some(&sr), name) {
							Ok(v) => {
								pt.args[i] = program::Arg::Int(v);
							}
							Err(ProcSymbolError::NotFound) => {}
							Err(err) => {
								return Err(err.to_error(token.sref));
							}
						}
					}
				}
				sr.add_token(pt);
			}
		}

		it.next();
	}

	if sr.tokens.len() == 0 || !sr.tokens.last().unwrap().is_end() {
		let sref;
		if let Some(v) = sr.tokens.last() {
			sref = v.sref;
		}
		else {
			sref = sref_start;
		}
		return Err(CompileError::new(sref, "Missing 'stop' or 'ret' statement".to_string()));
	}

	sr.add_symbol("!", sr.stack_size + 0x380);
	program.add_subroutine(sr);
	return Ok(());
}

