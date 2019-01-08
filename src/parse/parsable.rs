mod r#struct;
mod whitespace;
mod forced_eof;
mod comments;
mod number;
mod variable;
mod text;
mod oper;
mod block;

use crate::{Shared, Object, Error};
use crate::parse::Parser;
use lazy_static::lazy_static;

pub trait Parsable {
	const NAME: &'static str;
	fn try_parse(parser: &Shared<Parser>) -> ParseResult;
}

pub enum ParseResult {
	Restart, // for things like whitespace and comments
	Ok(Object),
	Err(Error),
	Eof, // for things like __END__
	None
}

pub use self::r#struct::ParsableStruct;

lazy_static! {
	pub static ref BUILTIN_PARSERS: Shared<Vec<ParsableStruct>> = Shared::new(vec!{
		ParsableStruct::new::<whitespace::Whitespace>(),
		ParsableStruct::new::<forced_eof::ForcedEof>(),
		ParsableStruct::new::<comments::Comments>(),
		ParsableStruct::new::<number::Number>(),
		ParsableStruct::new::<text::Text>(),
		ParsableStruct::new::<oper::Oper>(),
		ParsableStruct::new::<variable::Variable>(),
		ParsableStruct::new::<block::Block>(),
	});
}