mod basic;
mod pristine;

mod boolean;
mod null;
mod number;
mod text;
mod variable;
mod rustfn;
mod list;
mod map;
mod oper;

pub use self::{
	boolean::Boolean,
	null::Null,
	number::Number,
	text::Text,
	variable::Variable,
	rustfn::RustFn,
	list::List,
	map::Map,
	oper::Oper
};

pub(crate) use self::pristine::PRISTINE_MAP;

use crate::shared::Shared;
use crate::object::Object;
use crate::collections::{Collection, Mapping};
use std::fmt::{self, Display, Formatter};

#[cfg(not(feature = "fine-debug"))]
use std::fmt::Debug;


#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "fine-debug", derive(Debug))]
enum Types {
	Null,
	Boolean(Boolean),
	Number(Number),
	Text(Text),
	Variable(Variable),
	RustFn(RustFn),
	List(List),
	Map(Map),
	Oper(Oper)
}

trait Type : Into<Types> {
	fn create_mapping() -> Shared<dyn Mapping>;
}


#[derive(Clone)]//, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "fine-debug", derive(Debug))]
pub struct TypedObject {
	data: Types,
	map: Shared<dyn Mapping>
}

impl TypedObject {
	fn new<T: Type>(obj: T) -> Self {
		TypedObject {
			data: obj.into(),
			map: T::create_mapping(),
		}
	}

	pub fn objectify(self) -> Object {
		Object::new(self)
	}
}

impl Display for Types {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Types::Null => Display::fmt(&Null, f),
			Types::Boolean(ref bool) => Display::fmt(bool, f),
			Types::Number(ref num) => Display::fmt(num, f),
			Types::Text(ref text) => Display::fmt(text, f),
			Types::Variable(ref var) => Display::fmt(var, f),
			Types::RustFn(ref rustfn) => Display::fmt(rustfn, f),
			Types::List(ref list) => Display::fmt(list, f),
			Types::Map(ref map) => Display::fmt(map, f),
			Types::Oper(ref oper) => Display::fmt(oper, f),
		}
	}
}

#[cfg(not(feature = "fine-debug"))]
impl Debug for Types {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Types::Null => Debug::fmt(&Null, f),
			Types::Boolean(ref bool) => Debug::fmt(bool, f),
			Types::Number(ref num) => Debug::fmt(num, f),
			Types::Text(ref text) => Debug::fmt(text, f),
			Types::Variable(ref var) => Debug::fmt(var, f),
			Types::RustFn(ref rustfn) => Debug::fmt(rustfn, f),
			Types::List(ref list) => Debug::fmt(list, f),
			Types::Map(ref map) => Debug::fmt(map, f),
			Types::Oper(ref oper) => Debug::fmt(oper, f),
		}
	}
}

impl Display for TypedObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.data, f)
	}
}

#[cfg(not(feature = "fine-debug"))]
impl Debug for TypedObject {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			let mut ds = f.debug_struct("TypedObject");
			ds.field("data", &self.data);
			if !self.map.is_empty() {
				ds.field("map", &self.map);
			}
			return ds.finish();
		}

		if self.map.is_empty() {
			Debug::fmt(&self.data, f)
		} else {
			write!(f, "T{{ data: {:?}, map: {:?} }}", self.data, self.map)
		}
	}
}

impl Collection for TypedObject {
	fn len(&self) -> usize {
		self.map.len()
	}

	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}

impl Mapping for TypedObject {
	fn duplicate(&self) -> Shared<dyn Mapping> {
		Shared::new(TypedObject{ data: self.data.clone(), map: self.map.duplicate() })
	}

	fn get(&self, key: &Object) -> Option<Object> {
		self.map.get(key)
	}

	#[inline]
	fn set(&mut self, key: Object, val: Object) -> Option<Object> {
		self.map.set(key, val)
	}

	#[inline]
	fn del(&mut self, key: &Object) -> Option<Object> {
		self.map.del(key)
	}

	#[inline]
	fn has(&self, key: &Object) -> bool {
		self.map.has(key)
	}
}