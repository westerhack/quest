use std::fmt::{self, Debug, Formatter};
use crate::object::{Object, Type};

#[derive(Clone, Copy)]
pub struct Ops {
	pub eq: fn(&Object, &Object) -> bool,
	pub debug: fn(&Object, &mut Formatter) -> fmt::Result,
	pub clone: fn(&Object) -> Object
}

impl Ops {
	pub fn from<T: Eq + Debug + Clone + Send + Sync + 'static>() -> Ops {
		Ops {
			eq: |this, other|  unimplemented!(),
				// if let Some(other) = other.try_as_ref::<T>() {
				// 	this.try_as_ref::<T>().expect("'this' passed to `eq` wasn't of right type'") == other
				// } else {
				// 	false
				// },
			debug: |this, f| unimplemented!(),
				// Debug::fmt(this.try_as_ref::<T>().expect("'this' passed to `debug` wasn't of right type"), f),
			clone: |this| unimplemented!(),
				// Data::new(this.try_as_ref::<T>().expect("'this' passed to `clone` wasn't of right type") .clone())
		}
	}
}

impl Debug for Ops {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		struct PointerDebug(*const ());
		impl Debug for PointerDebug {
			fn fmt(&self, f: &mut Formatter) -> fmt::Result {
				write!(f, "{:p}", self.0)
			}
		}

		f.debug_struct("Ops")
			.field("eq", &PointerDebug(self.eq as *const ()))
			.field("debug", &PointerDebug(self.debug as *const ()))
			.field("clone", &PointerDebug(self.clone as *const ()))
			.finish()
	}
}

impl Eq for Ops {}
impl PartialEq for Ops {
	fn eq(&self, other: &Ops) -> bool {
		(self.eq as usize) == (other.eq as usize) &&
		(self.debug as usize) == (other.debug as usize) &&
		(self.clone as usize) == (other.clone as usize)
	}
}