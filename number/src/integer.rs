enum Datum{
	Byte(u8),
	Word(u16),
	Dword(u32),
	Qword(u64),
	Oword(u128)
}

pub struct Integer{
	is_signed:bool,
	datum:Datum
}


impl From<u8> for Integer{
	fn from(value: u8) -> Self {
		todo!()
	}
}

impl From<i8> for Integer{
	fn from(value: i8) -> Self {
		todo!()
	}
}

impl From<u16> for Integer{
	fn from(value: u16) -> Self {
		todo!()
	}
}

impl From<i16> for Integer{
	fn from(value: i16) -> Self {
		todo!()
	}
}

impl From<u32> for Integer{
	fn from(value: u32) -> Self {
		todo!()
	}
}

impl From<i32> for Integer{
	fn from(value: i32) -> Self {
		todo!()
	}
}

impl From<u64> for Integer{
	fn from(value: u64) -> Self {
		todo!()
	}
}

impl From<i64> for Integer{
	fn from(value: i64) -> Self {
		todo!()
	}
}

impl From<u128> for Integer{
	fn from(value: u128) -> Self {
		todo!()
	}
}

impl From<i128> for Integer{
	fn from(value: i128) -> Self {
		todo!()
	}
}