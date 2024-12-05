use core::fmt;

use crate::tokenizer::text::SourceRef;

pub struct CompileError<'a> {
	sref: SourceRef<'a>,
	msg: String,
}

fn get_pos(text: &str, index: usize) -> (&str, usize, usize) {
	let mut pos = (0, 1);
	let mut line_start = 0;

	for (index, ch) in text.char_indices().take(index) {
		match ch {
			'\n' => {
				pos.1 += 1;
				pos.0 = 0;
				line_start = index + 1;
			}
			_ => {
				pos.0 += 1;
			}
		}
	}

	let mut line = &text[line_start..];

	if let Some(end) = line.find('\n') {
		line = &line[..end];
	}

	return (line, pos.0, pos.1);
}

impl<'a> fmt::Display for CompileError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (line, x, y) = get_pos(self.sref.text, self.sref.start);
		let size;
		if self.sref.end > self.sref.start {
			size = self.sref.end - self.sref.start;
		}
		else {
			size = 0;
		}

		write!(f, "Error at {}:{}: {}\n{}\n", y, x, self.msg, line)?;

		for (index, ch) in line.char_indices() {
			if index > x + size {
				break;
			}
			if ch == '\t' {
				write!(f, "\t")?;
				continue;
			}
			if index >= x {
				write!(f, "^")?;
				continue;
			}
			write!(f, " ")?;
		}

		return Ok(());
	}
}

impl<'a> CompileError<'a> {
	pub fn new(sref: SourceRef<'a>, msg: String) -> CompileError {
		CompileError {
			sref,
			msg,
		}
	}
}

