
use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{error::CompileError, tokenizer::{text::SourceRef, token::Token}};

pub struct Metadata<'a> {
	pub sref: SourceRef<'a>,
	pub value: &'a str,
}

pub fn process<'a>(it: &mut Peekable<Iter<Token<'a>>>, metadata: &mut HashMap<&'a str, Metadata<'a>>) -> Result<(), CompileError<'a>> {
	loop {

		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};
		
		if token.name.ends_with(':') || token.name.starts_with('.') {
			break;
		}

		match (token.name, token.args.len()) {
			(name, 1) => {
				metadata.insert(name, Metadata {
					sref: token.sref,
					value: token.args[0],
				});
			}
			(_, count) => {
				return Err(CompileError::new(token.sref, format!("Bad argument count. Expected 1, got {}", count)));
			}
		}
		
		it.next();
	}

	return Ok(());
}

