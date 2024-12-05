use crate::{error::CompileError, program::Program, tokenizer::token::program::Arg};

use super::Subroutine;


impl<'a> Subroutine<'a> {
	pub fn finalize(&mut self, program: &mut Program<'a>, dst: &mut Vec<i32>) -> Result<(), CompileError<'a>> {

		for i in 0..self.tokens.len() {
			dst.push(self.tokens[i].itype);
			for j in 0..self.tokens[i].args.len() {
				if let Arg::Str(name) = self.tokens[i].args[j] {
					self.tokens[i].args[j] = match program.proc_symbol(Some(self), name) {
						Ok(v) => Arg::Int(v),
						Err(err) => return Err(err.to_error(self.tokens[i].sref)),
					};
				}
				if let Arg::Int(value) = self.tokens[i].args[j] {
					dst.push(value);
				}
				else {
					panic!();
				}
			}
		}

		return Ok(());
	}
}

