use std::{iter::Peekable, slice::Iter};

use crate::{error::CompileError, program::{ProcSymbolError, Program}, tokenizer::{text::SourceRef, token::{program::{self, ProgramToken}, Token}}};

use super::Subroutine;


pub fn process<'a>(sref_start: SourceRef<'a>, it: &mut Peekable<Iter<Token<'a>>>, program: &mut Program<'a>, name: &'a str) -> Result<Subroutine<'a>, CompileError<'a>> {
	let mut sr = Subroutine::new(name);
	sr.program_start = program.get_program_end();

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

		match (token.name, token.args.len()) {
			("local", 1) => {
				if token.args[0].starts_with('^') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				sr.add_stack(token.args[0], 1);
			}
			("local", 2) => {
				if token.args[0].starts_with('^') {
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
			("static", 1) => {
				if token.args[0].starts_with('^') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				sr.add_symbol(token.args[0], program.add_hidden_global(1));
			}
			("static", 2) => {
				if token.args[0].starts_with('^') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				let size = match program.proc_symbol(Some(&sr), &token.args[1]) {
					Ok(v) => v,
					Err(err) => return Err(err.to_error(token.sref)),
				};
				sr.add_symbol(token.args[0], program.add_hidden_global(size));
			}
			("label", 1) => {
				if token.args[0].starts_with('^') {
					return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
				}
				sr.add_symbol(token.args[0], sr.program_start + sr.program_size);
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

	sr.add_symbol("^", sr.stack_size + 0x380);
	program.add_subroutine(&sr);
	return Ok(sr);
}

