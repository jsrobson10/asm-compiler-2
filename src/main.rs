use std::fs;

use output::display::display_hex;

pub mod text;
pub mod tokenizer;
pub mod parser;
pub mod token;
pub mod subroutine;

mod compiler;
mod output;

fn main() {
	let vargs: Vec<String> = std::env::args().collect();
	let file = String::from_utf8(fs::read(&vargs[1]).unwrap()).unwrap();

	match compiler::process(&file) {
		Ok(binary) => {
			display_hex(&binary, [32, 16], 3).unwrap();
		}
		Err(err) => {
			println!("{}", err);
		}
	}
}
