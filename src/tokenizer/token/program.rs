
use crate::tokenizer::text::SourceRef;

use super::Token;

mod math;
mod inst;

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
		let name;
		let mtype;
		if let Some(dotpos) = token.name.find('.') {
			name = &token.name[..dotpos];
			let subaction = &token.name[dotpos+1..];
			mtype = match math::parse_subaction(subaction) {
				Some(v) => v,
				None => return Err(format!("Unknown subaction '{}'", subaction)),
			};
		}
		else {
			name = token.name;
			mtype = 0;
		}
		let (itype, arglen) = match inst::parse_instruction(name) {
			Some(v) => v,
			None => return Err(format!("Unknown instruction. Got '{}'", name)),
		};
		let mut pt = ProgramToken {
			sref: token.sref,
			args: Vec::with_capacity(token.args.len()),
			itype: itype << 8 | mtype,
		};
		for i in 0..token.args.len() {
			pt.args.push(Arg::Str(token.args[i]));
		}
		if arglen != token.args.len() {
			return Err(format!("Bad instruction count for '{}'. Expected {}, got {}", token.name, arglen, token.args.len()));
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

