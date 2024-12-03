use std::{iter::Peekable, str::CharIndices};

use crate::text::fmt_error;

#[derive(Debug)]
pub struct RawToken<'a> {
	pub index: usize,
	pub name: &'a str,
	pub subaction: Option<&'a str>,
	pub args: Vec<&'a str>,
}

fn skip_chars(it: &mut Peekable<CharIndices>, chars: &[char]) {
	while let Some((_, ch)) = it.peek() {
		if !chars.contains(ch) {
			break;
		}
		it.next();
	}
}

const WS: [char; 2] = [' ', '\t'];
const WS_AND_NL: [char; 3] = ['\n', '\t', ' '];
const SPECIAL: [char; 5] = ['\n', '\t', ' ', ',', '.'];

pub fn process(text: &str) -> Result<Vec<RawToken>, String> {
	let mut tokens: Vec<RawToken> = Vec::new();
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
			return Err(fmt_error(text, name_start, format!("Name length is 0")));
		}

		let mut token = RawToken {
			index: name_start,
			name: &text[name_start..name_end],
			subaction: None,
			args: Vec::new(),
		};
		
		skip_chars(&mut it, &WS);

		if it.next_if(|x| x.1 == '.').is_some() {
			skip_chars(&mut it, &WS);

			let subaction_start = match it.peek() {
				None => return Err(fmt_error(text, text.len(), format!("Expected subaction, got EOF"))),
				Some((i, _)) => *i,
			};
			let mut subaction_end = subaction_start;
			
			while let Some((i, _)) = it.next_if(|x| !SPECIAL.contains(&x.1)) {
				subaction_end = i + 1;
			}

			if subaction_start == subaction_end {
				return Err(fmt_error(text, subaction_start, format!("Subaction length is 0")));
			}

			token.subaction = Some(&text[subaction_start..subaction_end]);

			skip_chars(&mut it, &WS);
		}

		loop {
	
			let arg_start = match it.peek() {
				Some((_, '\n')) | None => break,
				Some((i, ',')) => return Err(fmt_error(text, *i, format!("Unexpected ','"))),
				Some((i, '.')) => return Err(fmt_error(text, *i, format!("Unexpected '.'"))),
				Some((i, _)) => *i,
			};
			let mut arg_end = arg_start;

			while let Some((i, _)) = it.next_if(|x| !SPECIAL.contains(&x.1)) {
				arg_end = i + 1;
			}
			
			skip_chars(&mut it, &WS);

			if arg_start < arg_end {
				token.args.push(&text[arg_start..arg_end]);
			}
			
			match it.peek() {
				Some((_, '\n')) | None => break,
				Some((_, ',')) => {
					it.next();
				}
				Some((i, ch)) => return Err(fmt_error(text, *i, format!("Expected ',' or newline, got '{}'", ch))),
			}
			
			skip_chars(&mut it, &WS);
		}

		tokens.push(token);
	}

	return Ok(tokens);
}

