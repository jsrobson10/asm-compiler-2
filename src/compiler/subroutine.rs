use std::{iter::Peekable, slice::Iter};

use crate::{parser::Parser, subroutine::Subroutine, token::Token, tokenizer::RawToken};

pub fn process<'a>(it: &mut Peekable<Iter<RawToken<'a>>>, parser: &mut Parser<'a>, name: &'a str) -> Result<(), String> {
	let mut sr = Subroutine::new(name);

	loop {
		let token = match it.peek() {
			Some(v) => v,
			None => break,
		};

		if token.name == ".section" {
			return Err("Section cannot follow sr".to_string());
		}

		match (&token.name[0..], &token.subaction, token.args.len()) {
			("stop", None, 0) => {
				sr.add_token(Token::Stop())?;
			}
			("set", None, 2) => {
				sr.add_token(Token::Set(sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?))?;
			}
			("set_store", None, 2) => {
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
			("math", Some(_subaction), 3) => { //TODO
				sr.add_token(Token::Math(0, sr.proc_symbol(parser, &token.args[0])?, sr.proc_symbol(parser, &token.args[1])?, sr.proc_symbol(parser, &token.args[2])?))?;
			}
			("jump", None, 1) => {
				sr.add_token(Token::Jump(token.args[0]))?;
			}
			("jump_if", None, 2) => {
				sr.add_token(Token::JumpIf(sr.proc_symbol(parser, &token.args[0])?, token.args[1]))?;
			}
			("jump_if_not", None, 2) => {
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
			}
			("ret", None, 1) => {
				sr.add_token(Token::Ret(sr.proc_symbol(parser, &token.args[0])?))?;
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

	parser.add_subroutine(sr);
	return Ok(());
}

