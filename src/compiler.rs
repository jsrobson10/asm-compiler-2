
mod subroutine;
mod section_data;
mod section_text;
mod section_global;
mod math;

use std::collections::HashMap;

use crate::{parser::Parser, text::{self}, tokenizer::{self}};

pub fn process(text: &str) -> Result<Vec<i32>, String> {

	let tokens = tokenizer::process(text)?;
	let mut parser = Parser::new();
	let mut it = tokens.iter().peekable();
	let mut metadata: HashMap<&str, &str> = HashMap::new();

	loop {
		let token = match it.next() {
			Some(v) => v,
			None => break,
		};
		
		if token.name.ends_with(":") && token.args.len() == 0 && token.subaction == None {
			if let Err(err) = subroutine::process(&mut it, &mut parser, &token.name[..token.name.len() - 1]) {
				let token = match it.peek() {
					Some(token) => token,
					None => tokens.last().unwrap_or(token),
				};
				return Err(text::fmt_error(text, token.index, err));
			}
			continue;
		}

		if let Err(err) = match (token.name, token.subaction, token.args.as_slice()) {
			(name, Some(subaction), _) => return Err(text::fmt_error(text, token.index, format!("Expected section or subroutine, got '{}.{}'", name, subaction))),
			(".section", None, ["data"]) => section_data::process(&mut it, &mut parser),
			(".section", None, ["global"]) => section_global::process(&mut it, &mut parser),
			(".section", None, ["text"]) => section_text::process(&mut it, &mut metadata),
			(".section", None, args) => return Err(text::fmt_error(text, token.index, format!("Unexpected section name, got '{}'", args.join(" ")))),
			(name, None, _) => return Err(text::fmt_error(text, token.index, format!("Expected section or subroutine, got '{}'", name))),
		} {
			let token = match it.peek() {
				Some(token) => token,
				None => tokens.last().unwrap_or(token),
			};
			return Err(text::fmt_error(text, token.index, err));
		}
	}

	return match parser.generate_binary(&metadata.get("global").unwrap_or(&"_start")) {
		Ok(v) => Ok(v),
		Err(err) => Err(format!("Error: {}", err)),
	};
}

