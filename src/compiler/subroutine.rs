use std::{iter::Peekable, slice::Iter};

use crate::{parser::Parser, subroutine::Subroutine, token::Token, tokenizer::RawToken};

use super::math;

pub fn process<'a>(it: &mut Peekable<Iter<RawToken<'a>>>, parser: &mut Parser<'a>, name: &'a str) -> Result<(), String> {
	let mut sr = Subroutine::new(name);
	let mut last_was_ret = false;

	loop {
		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};

		if token.name.starts_with('.') {
			return Err("Section cannot follow subroutine".to_string());
		}

		if token.name.ends_with(':') {
			break;
		}

		last_was_ret = false;

		match (&token.name[0..], &token.subaction, token.args.len()) {
			("stop", None, 0) => {
				sr.add_token(Token::Stop())?;
			}
			("set", None, 2) => {
				sr.add_token(Token::Set(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("set_s", None, 2) => {
				sr.add_token(Token::SetStore(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("copy", None, 2) => {
				sr.add_token(Token::Copy(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("swap", None, 2) => {
				sr.add_token(Token::Swap(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("store", None, 2) => {
				sr.add_token(Token::Store(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("load", None, 2) => {
				sr.add_token(Token::Load(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("math", Some(subaction), 3) => {
				let mathop = match math::parse_subaction(subaction) {
					None => return Err(format!("'{}' is not a valid math operation", subaction)),
					Some(v) => v,
				};
				sr.add_token(Token::Math(mathop, sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?, sr.proc_symbol(parser, &token.args[2])?))?;
			}
			("jump", None, 1) => {
				sr.add_token(Token::Jump(token.args[0]))?;
			}
			("jump_if", None, 2) => {
				sr.add_token(Token::JumpIf(sr.proc_symbol(parser, &token.args[0])?, token.args[1]))?;
			}
			("jump_iz", None, 2) => {
				sr.add_token(Token::JumpIfNot(sr.proc_symbol(parser, &token.args[0])?, token.args[1]))?;
			}
			("push", None, 1) => {
				sr.add_token(Token::Push(sr.proc_symbol(parser, &token.args[0])?))?;
			}
			("call", None, 1) => {
				sr.add_token(Token::Call(token.args[0], 3))?;
			}
			("call", None, 2) => {
				sr.add_token(Token::Call(token.args[0], sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("ret", None, 0) => {
				sr.add_token(Token::Ret(3))?;
				last_was_ret = true;
			}
			("ret", None, 1) => {
				sr.add_token(Token::Ret(sr.proc_symbol(parser, &token.args[0])?))?;
				last_was_ret = true;
			}
			("nop", None, 0) => {
				sr.add_token(Token::Nop())?;
			}
			("local", None, 1) => {
				sr.add_token(Token::Local(token.args[0], 1))?;
			}
			("local", None, 2) => {
				sr.add_token(Token::Local(token.args[0], sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("local_set", None, 2) => {
				sr.add_token(Token::LocalSet(token.args[0], sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("label", None, 1) => {
				sr.add_token(Token::Label(token.args[0]))?;
			}
			(name, None, _) => {
				return Err(format!("Bad instruction '{}'", name));
			}
			(name, Some(subaction), _) => {
				return Err(format!("Bad instruction '{}'.'{}'", name, subaction));
			}
		}

		it.next();
	}

	if !last_was_ret {
		return Err(format!("Missing return statement"));
	}

	parser.add_subroutine(sr);
	return Ok(());
}

