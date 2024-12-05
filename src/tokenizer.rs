use text::{skip_chars, SourceRef};
use token::Token;

use crate::error::CompileError;


pub mod token;
pub mod text;

const WS: [char; 2] = [' ', '\t'];
const WS_AND_NL: [char; 3] = ['\n', '\t', ' '];
const SPECIAL: [char; 4] = ['\n', '\t', ' ', ','];

pub fn process(text: &str) -> Result<Vec<Token>, CompileError> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut it = text.char_indices().peekable();

	loop {
		skip_chars(&mut it, &WS_AND_NL);

		let name_start = match it.peek() {
			None => break,
			Some(&(i, '.')) => {
				it.next();
				i
			},
			Some(&(i, _)) => i,
		};
		let mut name_end = name_start;

		while let Some((i, _)) = it.next_if(|x| !SPECIAL.contains(&x.1)) {
			name_end = i + 1;
		}

		if name_start == name_end {
			return Err(CompileError::new(SourceRef::new(text, name_start, name_end), format!("Name length is 0")));
		}

		let mut token = Token {
			sref: SourceRef::new(text, name_start, name_end - 1),
			name: &text[name_start..name_end],
			args: Vec::new(),
		};
		
		skip_chars(&mut it, &WS);

		loop {
	
			let arg_start = match it.peek() {
				Some((_, '\n')) | None => break,
				Some((i, ',')) => return Err(CompileError::new(SourceRef::new(text, *i, *i), format!("Unexpected ','"))),
				Some((i, '.')) => return Err(CompileError::new(SourceRef::new(text, *i, *i), format!("Unexpected '.'"))),
				Some((i, _)) => *i,
			};
			let mut arg_end = arg_start;

			while let Some((i, _)) = it.next_if(|x| !SPECIAL.contains(&x.1)) {
				arg_end = i + 1;
			}
			
			token.sref.end = arg_end - 1;
			skip_chars(&mut it, &WS);

			if arg_start < arg_end {
				token.args.push(&text[arg_start..arg_end]);
			}
			
			match it.peek() {
				Some((_, '\n')) | None => break,
				Some((_, ',')) => {
					it.next();
				}
				Some((i, ch)) => return Err(CompileError::new(SourceRef::new(text, *i, *i), format!("Expected ',' or newline, got '{}'", ch))),
			}
			
			skip_chars(&mut it, &WS);
		}

		tokens.push(token);
	}

	return Ok(tokens);
}

