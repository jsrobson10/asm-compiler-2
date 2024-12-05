use std::fs;

use output::{display::display_hex, schematic};

pub mod error;
pub mod program;
pub mod tokenizer;
pub mod parser;

mod compiler;
mod output;

fn main() {
	let vargs: Vec<String> = std::env::args().collect();
	let file = String::from_utf8(fs::read(&vargs[1]).unwrap()).unwrap();

	match compiler::process(&file) {
		Ok(binary) => {
			println!("{:?}", binary);
			display_hex(&binary, [32, 16], 3).unwrap();
			if vargs.len() > 2 {
				let filename = &vargs[2];
				if let Err(error) = schematic::write(&binary, &filename, &schematic::WriteConfig::default()) {
					println!("Schematic Error: {}", error);
					return;
				}
				println!("Written schematic to '{}'", filename);
			}
		}
		Err(err) => {
			println!("{}", err);
		}
	}
}
