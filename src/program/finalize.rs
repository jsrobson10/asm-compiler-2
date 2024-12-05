use std::collections::HashMap;

use crate::{error::CompileError, parser::{section::text::Metadata, subroutine::Subroutine}};

use super::Program;


impl<'a> Program<'a> {
	pub fn finalize(&mut self, metadata: &HashMap<&'a str, Metadata<'a>>, subroutines: &mut Vec<Subroutine<'a>>) -> Result<Vec<i32>, CompileError<'a>> {
		let mut dst = Vec::with_capacity(0x200);
		let meta_start = &metadata["global"];
		let start = match self.proc_symbol(None, &meta_start.value) {
			Ok(v) => v,
			Err(err) => return Err(err.to_error(meta_start.sref)),
		};

		dst.extend_from_slice(&[0xd00, 0, self.proc_inline_constant(start), 0xfff, 0]);

		for subroutine in subroutines.iter_mut() {
			subroutine.finalize(self, &mut dst)?;
		}
		
		assert_eq!(dst.len() as i32, self.program_end);

		dst.resize(0x200 - self.constants.len(), 0);
		dst.extend(self.constants.iter().rev());

		return Ok(dst);
	}
}
