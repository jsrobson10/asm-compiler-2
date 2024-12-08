use core::fmt;
use std::{iter::Peekable, str::CharIndices};

#[derive(Copy,Clone)]
pub struct SourceRef<'a> {
	pub text: &'a str,
	pub start: usize,
	pub end: usize,
}

impl<'a> fmt::Debug for SourceRef<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "SourceRef [{}..{}]", self.start, self.end)
	}
}

impl<'a> SourceRef<'a> {
	pub fn new(text: &'a str, start: usize, end: usize) -> SourceRef {
		SourceRef {
			text,
			start,
			end,
		}
	}
}

pub fn skip_until_chars(it: &mut Peekable<CharIndices>, chars: &[char]) {
	while let Some(&(_, ch)) = it.peek() {
		if chars.contains(&ch) {
			break;
		}
		it.next();
	}
}

pub fn skip_chars(it: &mut Peekable<CharIndices>, chars: &[char]) {
	while let Some(&(_, ch)) = it.peek() {
		if ch == ';' {
			skip_until_chars(it, &['\n']);
		}
		if !chars.contains(&ch) {
			break;
		}
		it.next();
	}
}
