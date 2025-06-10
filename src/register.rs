use lcid::constants::lcid::LCID_KO_KR;
use windows::Win32::{
	Foundation::MAX_PATH,
	System::LibraryLoader::GetModuleFileNameW,
	UI::TextServices::{
		CLSID_TF_CategoryMgr, CLSID_TF_InputProcessorProfiles,
		GUID_TFCAT_TIP_KEYBOARD, GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
		GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, ITfCategoryMgr,
		ITfInputProcessorProfileMgr,
	},
};

use crate::clsid::ClsId;
use crate::meta;
use crate::prelude::*;

pub fn get_module_path(instance: HINSTANCE) -> OsString {
	let mut path = [0; MAX_PATH as usize];
	let path_len =
		unsafe { GetModuleFileNameW(Some(instance.into()), &mut path) };

	OsString::from_wide(if path_len >= MAX_PATH {
		&path[0..(MAX_PATH - 1) as usize]
	} else {
		&path[0..path_len as usize]
	})
}

pub trait Registerable {
	const INIT: Self;
	type RegisterParam;
	fn register(&self, param: &Self::RegisterParam) -> WinResult<()>;
	fn unregister(&self) -> WinResult<()>;
}

pub struct Profile {
	clsid: ClsId,
	langid: u16,
	profile_id: GUID,
	name: &'static str,
	icon_index: u32,
	enable_by_default: bool,
}

impl Profile {
	unsafe fn get_profile_mgr() -> WinResult<ITfInputProcessorProfileMgr> {
		unsafe { co_create_inproc(&CLSID_TF_InputProcessorProfiles) }
	}
}

impl Registerable for Profile {
	const INIT: Self = Self {
		clsid: meta::CLSID,
		langid: LCID_KO_KR as _,
		profile_id: meta::PROFILE_GUID,
		name: meta::NAME,
		icon_index: 0u32.wrapping_sub(101),
		enable_by_default: true,
	};

	type RegisterParam = OsString;

	fn register(&self, module_path: &OsString) -> WinResult<()> {
		let profile_mgr = unsafe { Self::get_profile_mgr() }?;

		let module_path: Box<[u16]> = module_path.encode_wide().collect();
		let name: Box<[u16]> = self.name.encode_utf16().collect();

		unsafe {
			profile_mgr.RegisterProfile(
				&self.clsid.guid,
				self.langid,
				&self.profile_id,
				&name,
				&module_path,
				self.icon_index,
				Default::default(),
				0,
				self.enable_by_default,
				0,
			)
		}?;

		Ok(())
	}

	fn unregister(&self) -> WinResult<()> {
		let profile_mgr = unsafe { Self::get_profile_mgr() }?;

		unsafe {
			profile_mgr.UnregisterProfile(
				&self.clsid.guid,
				self.langid,
				&self.profile_id,
				0,
			)
		}
	}
}

pub struct Registry {
	clsid: ClsId,
	name: &'static str,
}

impl Registerable for Registry {
	const INIT: Self = Self {
		clsid: meta::CLSID,
		name: meta::NAME,
	};

	type RegisterParam = OsString;
	fn register(&self, module_path: &OsString) -> WinResult<()> {
		let key = windows_registry::CLASSES_ROOT
			.create(self.clsid.to_registry_key())?;

		key.set_string("", self.name)?;

		let key = key.create("InprocServer32")?;
		key.set_string("", module_path.to_string_lossy())?;
		key.set_string("ThreadingModel", "Apartment")?;

		Ok(())
	}

	fn unregister(&self) -> WinResult<()> {
		windows_registry::CLASSES_ROOT.remove_tree(self.clsid.to_registry_key())
	}
}

pub struct Category {
	clsid: ClsId,
	categories: &'static [GUID],
}

impl Category {
	unsafe fn get_category_mgr() -> WinResult<ITfCategoryMgr> {
		unsafe { co_create_inproc(&CLSID_TF_CategoryMgr) }
	}
}

impl Registerable for Category {
	type RegisterParam = ();
	const INIT: Self = Self {
		clsid: meta::CLSID,
		categories: &[
			GUID_TFCAT_TIP_KEYBOARD, // keyboard input method
			GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT, // metro mode
			GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, // win8+ systray
		],
	};

	fn register(&self, _: &()) -> WinResult<()> {
		let mgr = unsafe { Self::get_category_mgr() }?;
		for id in self.categories {
			unsafe {
				mgr.RegisterCategory(&self.clsid.guid, id, &self.clsid.guid)
			}?;
		}
		Ok(())
	}

	fn unregister(&self) -> WinResult<()> {
		let mgr = unsafe { Self::get_category_mgr() }?;

		let mut first_failed_result = Ok(());
		for id in self.categories {
			let result = unsafe {
				mgr.UnregisterCategory(&self.clsid.guid, id, &self.clsid.guid)
			};

			if first_failed_result.is_ok() && result.is_err() {
				first_failed_result = result;
			}
		}

		first_failed_result
	}
}

pub struct All;

impl Registerable for All {
	const INIT: Self = Self;
	type RegisterParam = OsString;

	fn register(&self, module_path: &OsString) -> WinResult<()> {
		Registry::INIT.register(module_path)?;
		Profile::INIT.register(module_path)?;
		Category::INIT.register(&())?;
		Ok(())
	}

	fn unregister(&self) -> WinResult<()> {
		let a = Category::INIT.unregister();
		let b = Profile::INIT.unregister();
		let c = Registry::INIT.unregister();
		a?;
		b?;
		c?;

		Ok(())
	}
}
