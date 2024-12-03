
#[derive(Debug,Clone)]
pub enum Token<'a> {
	Stop(),
	Set(i32, i32),
	SetStore(i32, i32),
	Copy(i32, i32),
	Swap(i32, i32),
	Store(i32, i32),
	Load(i32, i32),
	Math(i32, i32, i32, i32),
	Jump(&'a str),
	JumpIf(i32, &'a str),
	JumpIfNot(i32, &'a str),
	Push(i32),
	Call(&'a str, i32),
	Ret(i32),
	Nop(),

	Local(&'a str, i32),
	LocalSet(&'a str, i32),
	Label(&'a str),
}

impl Token<'_> {
	pub fn size(&self) -> i32 {
		match self {
			Token::Stop() => 0,
			Token::Set(_,_) => 3,
			Token::SetStore(_,_) => 3,
			Token::Copy(_,_) => 3,
			Token::Swap(_,_) => 3,
			Token::Store(_,_) => 3,
			Token::Load(_,_) => 3,
			Token::Math(_,_,_,_) => 4,
			Token::Jump(_) => 2,
			Token::JumpIf(_,_) => 3,
			Token::JumpIfNot(_,_) => 3,
			Token::Push(_) => 3,
			Token::Call(_,_) => 5,
			Token::Ret(_) => 2,
			Token::Nop() => 1,
			Token::LocalSet(_, _) => 3,
			_ => 0,
		}
	}
}

