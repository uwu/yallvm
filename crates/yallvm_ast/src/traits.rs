pub trait AstToBox
where
	Self: Sized,
{
	fn to_box(self) -> Box<Self> {
		Box::new(self)
	}
}

pub trait AstToEnum<T> {
	fn to_enum(self) -> T;
}
