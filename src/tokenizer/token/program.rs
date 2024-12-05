
use crate::tokenizer::text::SourceRef;

use super::Token;

mod math;

#[derive(Debug)]
pub enum Arg<'a> {
	Str(&'a str),
	Int(i32),
}

#[derive(Debug)]
pub struct ProgramToken<'a> {
	pub sref: SourceRef<'a>,
	pub itype: i32,
	pub args: Vec<Arg<'a>>,
}

impl<'a> ProgramToken<'a> {
	pub fn from(token: &Token<'a>) -> Result<ProgramToken<'a>, String> {
		let (itype, args) = match token.name {
			"stop" => (0x0, 0),
			"set" => (0x1, 2),
			"set_s" => (0x2, 2),
			"copy" => (0x3, 2),
			"swap" => (0x4, 2),
			"store" => (0x5, 2),
			"load" => (0x6, 2),
			"math" => (0x7, 3),
			"jump" => (0x8, 1),
			"jump_if" => (0x9, 2),
			"jump_z" => (0xa, 2),
			"call" => (0xd, 3),
			"ret" => (0xe, 1),
			"nop" => (0xf, 0),
			_ => return Err(format!("Unknown instruction. Got '{}'", token.name)),
		};
		let mut pt = ProgramToken {
			sref: token.sref,
			args: Vec::with_capacity(token.args.len()),
			itype,
		};
		for i in 0..token.args.len() {
			pt.args.push(Arg::Str(token.args[i]));
		}
		if args != token.args.len() {
			return Err(format!("Bad instruction count for '{}'. Expected {}, got {}", token.name, args, token.args.len()));
		}
		if let Some(subaction) = token.subaction {
			pt.itype |= match math::parse_subaction(subaction) {
				Some(v) => v,
				None => return Err(format!("Unknown subaction '{}'", subaction)),
			};
		}
		return Ok(pt);
	}
	pub fn size(&self) -> usize {
		return self.args.len() + 1;
	}
	pub fn is_end(&self) -> bool {
		match self.itype & 0xf00 {
			0x000 => true,
			0xe00 => true,
			_ => false,
		}
	}
}

