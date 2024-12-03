
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

		if token.args.len() != 1 {
			return Err(format!("Global '{}' must have 1 argument, got {}", token.args[0], token.args.len()));
		}

		let value = token.args[0];

		match &token.name[..1] {
			"@" => {
				let v = parser.proc_symbol(&value)?;
				parser.set_symbol(&token.name[1..], v);
			}
			_ => {
				let v = parser.proc_symbol(&value)?;
				parser.add_global(token.name, v);
			}
		}

		it.next();
	}
	return Ok(());
}

