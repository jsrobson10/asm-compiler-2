use super::text::SourceRef;

pub mod program;

#[derive(Debug)]
pub struct Token<'a> {
	pub sref: SourceRef<'a>,
	pub name: &'a str,
	pub args: Vec<&'a str>,
}

