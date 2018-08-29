use env::{Environment, parse::{Parsable, Token}};
use obj::{AnyObject, SharedObject};

use std::fmt::{self, Display, Formatter};

pub type QBool = SharedObject<bool>;

impl Parsable for bool {
	fn try_parse(env: &mut Environment) -> Option<Token> {
		match env.stream.try_get(regex!(r"\A([tT]rue|[fF]alse)\b"))? {
			"true"  | "True"  => Some(true.into()),
			"false" | "False" => Some(false.into()),
			other => unreachable!("found non-bool regex value `{:?}`", other)
		}
	}
}

define_attrs! { for QBool;
	use QObject<bool>;

	fn "@num" (this) {
		Ok(QNum::from_number(**this as u8))
	}

	fn "@text" (this) {
		Ok(QText::from(this.to_string()))
	}

	fn "@bool" () with _env _args obj {
		Ok(obj.clone())
	}
}