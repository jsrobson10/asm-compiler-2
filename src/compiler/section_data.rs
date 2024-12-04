
use std::{iter::Peekable, slice::Iter};

use crate::{parser::Parser, tokenizer::RawToken};

pub fn process<'a>(it: &mut Peekable<Iter<RawToken<'a>>>, parser: &mut Parser<'a>) -> Result<(), String> {
	loop {
		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};

		if token.name.ends_with(':') || token.name.starts_with('.') {
			break;
		}

		if token.subaction != None {
			return Err(format!("Unexpected '.' after '{}'", token.name));
		}

		let mut args: Vec<i32> = Vec::new();
		for arg in token.args.iter() {
			args.push(parser.proc_symbol(&arg, None)?);
		}

		parser.add_constant(&token.name, &args);
		it.next();
	}
	return Ok(());
}

