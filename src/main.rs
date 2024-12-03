use std::fs;

pub mod text;
pub mod tokenizer;
pub mod parser;
pub mod token;
pub mod subroutine;
pub mod compiler;

fn main() {
/*	let mut parser = Parser::new();

	parser.add_constant("numbers".to_string(), &[1, 2, 3, 4, 5]);
	let mut sr = Subroutine::new("main".to_string());
	sr.add_token(Token::Copy(parser.proc_symbol("numbers").unwrap(), parser.proc_symbol("0x401").unwrap())).unwrap();
	sr.add_token(Token::Copy(parser.proc_symbol("&0x123").unwrap(), parser.proc_symbol("0x402").unwrap())).unwrap();
	parser.add_subroutine(sr);
	println!("{:?}", parser);
	println!("{:?}", parser.generate_binary("main"));*/

	let vargs: Vec<String> = std::env::args().collect();
	let file = String::from_utf8(fs::read(&vargs[1]).unwrap()).unwrap();

	match compiler::process(&file) {
		Ok(binary) => {
			println!("{:?}", binary);
		}
		Err(err) => {
			println!("{}", err);
		}
	}
}
