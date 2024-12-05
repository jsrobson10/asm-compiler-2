
use std::collections::HashMap;

use crate::{error::CompileError, parser::{self, section::text::Metadata, subroutine::Subroutine}, program::Program, tokenizer::{self, text::SourceRef}};

pub fn process(text: &str) -> Result<Vec<i32>, CompileError> {

	let tokens = tokenizer::process(text)?;
	let mut it = tokens.iter().peekable();
	let mut metadata: HashMap<&str, Metadata> = HashMap::new();
	let mut subroutines: Vec<Subroutine> = Vec::new();
	let mut program = Program::new();

	loop {
		let token = match it.next() {
			Some(v) => v,
			None => break,
		};
		
		if token.name.ends_with(":") && token.args.len() == 0 {
			subroutines.push(parser::subroutine::process::process(token.sref, &mut it, &mut program, &token.name[..token.name.len() - 1])?);
			continue;
		}

		match (token.name, token.args.as_slice()) {
			(".section", ["data"]) => parser::section::data::process(&mut it, &mut program)?,
			(".section", ["global"]) => parser::section::global::process(&mut it, &mut program)?,
			(".section", ["text"]) => parser::section::text::process(&mut it, &mut metadata)?,
			(".section", args) => return Err(CompileError::new(token.sref, format!("Unexpected section name, got '{}'", args.join(" ")))),
			(name, _) => return Err(CompileError::new(token.sref, format!("Expected section or subroutine, got '{}'", name))),
		}
	}

	if metadata.get("global").is_none() {
		return Err(CompileError::new(SourceRef::new(text, 0, text.len()), format!("Entrypoint missing")));
	}

	return Ok(program.finalize(&metadata, &mut subroutines)?);
}

