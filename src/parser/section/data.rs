
use std::{iter::Peekable, slice::Iter};

use crate::{error::CompileError, program::Program, tokenizer::token::Token};

pub fn process<'a>(it: &mut Peekable<Iter<Token<'a>>>, program: &mut Program<'a>) -> Result<(), CompileError<'a>> {
	loop {
		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};

		if token.name.ends_with(':') || token.name.starts_with('.') {
			break;
		}

		if token.subaction != None {
			return Err(CompileError::new(token.sref, format!("Unexpected '.' after '{}'", token.name)));
		}
		
		if token.args[0].starts_with('!') {
			return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
		}

		let mut args: Vec<i32> = Vec::new();
		for arg in token.args.iter() {
			args.push(match program.proc_symbol(None, &arg) {
				Ok(v) => v,
				Err(e) => return Err(e.to_error(token.sref)),
			});
		}

		program.add_constant(&token.name, &args);
		it.next();
	}
	return Ok(());
}

