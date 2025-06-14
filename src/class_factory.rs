use windows::Win32::System::Com::{
	CoLockObjectExternal, IClassFactory, IClassFactory_Impl,
};
use windows_core::GUID;

use crate::{prelude::*, text_service::TextService};

use std::ffi::c_void;

#[implement(IClassFactory)]
#[derive(Default)]
pub struct ComFactory;

impl IClassFactory_Impl for ComFactory_Impl {
	fn CreateInstance(
		&self,
		_punkouter: WinRef<'_, IUnknown>,
		riid: *const GUID,
		ppvobject: *mut *mut c_void,
	) -> WinResult<()> {
		let text_service: IUnknown =
			TextService::new().into_object().into_interface();
		let _result = unsafe { text_service.query(riid, ppvobject) };

		Ok(())
	}
	fn LockServer(&self, flock: BOOL) -> WinResult<()> {
		unsafe {
			CoLockObjectExternal(self.as_interface_ref(), flock.as_bool(), true)
		}
	}
}
