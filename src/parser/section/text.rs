
use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{error::CompileError, tokenizer::token::Token};

pub fn process<'a>(it: &mut Peekable<Iter<Token<'a>>>, metadata: &mut HashMap<&'a str, &'a str>) -> Result<(), CompileError<'a>> {
	loop {

		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};
		
		if token.name.ends_with(':') || token.name.starts_with('.') {
			break;
		}

		match (token.name, token.subaction, token.args.len()) {
			(name, None, 1) => {
				metadata.insert(name, token.args[0]);
			}
			(name, Some(_), _) => {
				return Err(CompileError::new(token.sref, format!("Unexpected '.' after '{}'", name)));
			}
			(name, None, _) => {
				return Err(CompileError::new(token.sref, format!("Unexpected ',' after '{} {}'", name, token.args[0])));
			}
		}
		
		it.next();
	}

	return Ok(());
}

