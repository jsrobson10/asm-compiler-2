
use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::tokenizer::RawToken;

pub fn process<'a>(it: &mut Peekable<Iter<RawToken<'a>>>, metadata: &mut HashMap<&'a str, &'a str>) -> Result<(), String> {
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
				return Err(format!("Unexpected '.' after '{}'", name));
			}
			(name, None, _) => {
				return Err(format!("Unexpected ',' after '{} {}'", name, token.args[0]));
			}
		}
		
		it.next();
	}

	return Ok(());
}

