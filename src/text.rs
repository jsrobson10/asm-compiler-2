
fn get_pos(text: &str, index: usize) -> (String, usize, usize) {
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

	let mut indicator = String::new();
	for (index, ch) in line.char_indices() {
		if index == pos.0 {
			indicator.push('^');
			break;
		}
		if ch == '\t' {
			indicator.push('\t');
			continue;
		}
		indicator.push(' ');
	}

	return (format!("{}\n{}", line, indicator), pos.0, pos.1);
}

pub fn fmt_error(text: &str, index: usize, error: String) -> String {
	let (line, x, y) = get_pos(text, index);
	return format!("Error at {}:{}: {}\n{}", y, x, error, line);
}

