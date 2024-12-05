
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

		if token.args.len() != 1 {
			return Err(CompileError::new(token.sref, format!("Global '{}' must have 1 argument, got {}", token.args[0], token.args.len())));
		}
		
		if token.args[0].starts_with('^') {
			return Err(CompileError::new(token.sref, "Reserved keyword".to_string()));
		}

		let value = token.args[0];

		match &value[..1] {
			"@" => {
				let v = match program.proc_symbol(None, &value[1..]) {
					Ok(v) => v,
					Err(err) => return Err(err.to_error(token.sref)),
				};
				program.add_global(token.name, v);
			}
			_ => {
				let v = match program.proc_symbol(None, &value) {
					Ok(v) => v,
					Err(err) => return Err(err.to_error(token.sref)),
				};
				program.add_global(token.name, v);
			}
		}

		it.next();
	}
	return Ok(());
}

