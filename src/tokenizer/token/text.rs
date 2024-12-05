use crate::tokenizer::text::SourceRef;

use super::Token;


#[derive(Debug)]
pub struct TextToken<'a> {
	pub sref: SourceRef<'a>,
	pub name: &'a str,
	pub value: &'a str,
}

impl<'a> TextToken<'a> {
	pub fn from(token: &Token<'a>) -> Option<TextToken<'a>> {
		if token.subaction.is_some() || token.args.len() != 1 {
			return None;
		}
		return Some(TextToken {
			sref: token.sref,
			name: token.name,
			value: token.args[0],
		});
	}
}

