
use std::collections::HashMap;

use crate::{error::CompileError, parser, program::Program, tokenizer::{self}};

pub fn process(text: &str) -> Result<Vec<i32>, CompileError> {

	let tokens = tokenizer::process(text)?;
	let mut it = tokens.iter().peekable();
	let mut metadata: HashMap<&str, &str> = HashMap::new();
	let mut program = Program::new();

	loop {
		let token = match it.next() {
			Some(v) => v,
			None => break,
		};
		
		if token.name.ends_with(":") && token.args.len() == 0 && token.subaction == None {
			parser::subroutine::process(token.sref, &mut it, &mut program, &token.name[..token.name.len() - 1])?;
			continue;
		}

		match (token.name, token.subaction, token.args.as_slice()) {
			(name, Some(subaction), _) => return Err(CompileError::new(token.sref, format!("Expected section or subroutine, got '{}.{}'", name, subaction))),
			(".section", None, ["data"]) => parser::section::data::process(&mut it, &mut program)?,
			(".section", None, ["global"]) => parser::section::global::process(&mut it, &mut program)?,
			(".section", None, ["text"]) => parser::section::text::process(&mut it, &mut metadata)?,
			(".section", None, args) => return Err(CompileError::new(token.sref, format!("Unexpected section name, got '{}'", args.join(" ")))),
			(name, None, _) => return Err(CompileError::new(token.sref, format!("Expected section or subroutine, got '{}'", name))),
		}
	}

	println!("{:?}", program);

/*	return match parser.generate_binary(&metadata.get("global").unwrap_or(&"_start")) {
		Ok(v) => Ok(v),
		Err(err) => Err(format!("Error: {}", err)),
	};*/
	return Ok(Vec::new());//TODO
}

