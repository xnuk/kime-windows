pub use std::ffi::{OsStr, OsString, c_void};
pub use std::marker::PhantomData;
pub use std::os::windows::ffi::{OsStrExt, OsStringExt};
pub use std::{mem, ptr};

pub use windows::Win32::Foundation::{
	CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_FAIL, E_NOINTERFACE,
	E_UNEXPECTED, HINSTANCE, LPARAM, S_FALSE, S_OK, WPARAM,
};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
pub use windows::core::{
	BOOL, ComObjectInner, ComObjectInterface, GUID, HRESULT, HSTRING, IUnknown,
	Interface as WinInterface, InterfaceRef, Ref as WinRef,
	Result as WinResult, implement,
};

pub unsafe fn co_create_inproc<I: WinInterface>(guid: &GUID) -> WinResult<I> {
	unsafe { CoCreateInstance(guid, None, CLSCTX_INPROC_SERVER) }
}
