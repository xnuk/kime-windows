use windows::core::GUID;

use crate::clsid::ClsId;

// GUIDs
// Literally randomly choose from ANY guid generators

pub const CLSID: ClsId =
	ClsId::from_u128(0x54212921_51B5_4853_88C9_466A3C423218);

pub const PROFILE_GUID: GUID =
	GUID::from_u128(0x24777984_E165_492C_BD7E_18FA11E95F04);

pub const NAME: &str = env!("CARGO_CRATE_NAME");
