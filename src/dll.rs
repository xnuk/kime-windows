use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

use crate::{
	prelude::*,
	register::{self, Registerable, get_module_path},
};
use std::ptr;

static mut DLL_INSTANCE: HINSTANCE = HINSTANCE(ptr::null_mut());

#[unsafe(no_mangle)]
pub extern "system" fn DllRegisterServer() -> HRESULT {
	let module_path = unsafe { get_module_path(DLL_INSTANCE) };
	register::All::INIT.register(&module_path).into()
}

#[unsafe(no_mangle)]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
	register::All::INIT.unregister().into()
}

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(
	dll_instance: HINSTANCE,
	reason: u32,
	_: *mut c_void,
) -> BOOL {
	if reason == DLL_PROCESS_ATTACH {
		unsafe { DLL_INSTANCE = dll_instance };
	}
	true.into()
}

#[unsafe(no_mangle)]
pub extern "system" fn DllGetClassObject(
	rclsid: *const GUID,
	riid: *const GUID,
	pout: *mut *mut c_void,
) -> HRESULT {
	Ok(()).into()
}

#[unsafe(no_mangle)]
extern "system" fn DllCanUnloadNow() -> HRESULT {
	Ok(()).into()
}
