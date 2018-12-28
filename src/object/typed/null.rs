use super::{TypedObject, Type, Types};
use crate::{Shared, Object};
use crate::collections::{Mapping, ParentalMap};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Null;

impl Type for Null {
	fn create_mapping() -> Shared<dyn Mapping> {
		lazy_static! {
			static ref PARENT: Shared<Object> = Shared::new({
				unimplemented!();
			});
		}
		Shared::new(ParentalMap::new_default(PARENT.clone()))
	}
}

impl From<Null> for Types {
	fn from(_: Null) -> Types {
		Types::Null
	}
}


impl TypedObject {
	pub fn new_null() -> Self {
		TypedObject::new(Null)
	}
}