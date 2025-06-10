use windows::core::GUID;

#[repr(transparent)]
pub struct ClsId {
	pub guid: GUID,
}

impl ClsId {
	pub const fn from_u128(x: u128) -> Self {
		ClsId {
			guid: GUID::from_u128(x),
		}
	}

	pub fn to_registry_key(&self) -> String {
		format!("CLSID\\{{{:?}}}", self.guid)
	}
}

impl From<ClsId> for GUID {
	fn from(value: ClsId) -> Self {
		value.guid
	}
}

impl AsRef<GUID> for ClsId {
	fn as_ref(&self) -> &GUID {
		&self.guid
	}
}
