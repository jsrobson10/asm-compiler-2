use crate::tokenizer::text::SourceRef;

use super::Token;



#[derive(Debug)]
pub struct DataToken<'a> {
	pub sref: SourceRef<'a>,
	pub name: &'a str,
	pub args: Vec<&'a str>,
}

impl<'a> DataToken<'a> {
	pub fn from(token: &Token<'a>) -> Option<DataToken<'a>> {
		if token.subaction.is_some() {
			return None;
		}
		return Some(DataToken {
			sref: token.sref,
			name: token.name,
			args: token.args.clone(),
		});
	}
}

