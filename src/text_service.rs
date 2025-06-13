use std::{
	cell::Cell,
	sync::{RwLock, RwLockWriteGuard},
};

use windows::Win32::UI::{
	Input::KeyboardAndMouse::GetKeyboardState,
	TextServices::{
		ITfComposition, ITfCompositionSink, ITfCompositionSink_Impl,
		ITfContext, ITfDocumentMgr, ITfEditRecord, ITfFunctionProvider,
		ITfFunctionProvider_Impl, ITfKeyEventSink, ITfKeyEventSink_Impl,
		ITfKeystrokeMgr, ITfSource, ITfSourceSingle, ITfTextEditSink,
		ITfTextEditSink_Impl, ITfTextInputProcessor,
		ITfTextInputProcessor_Impl, ITfTextInputProcessorEx,
		ITfTextInputProcessorEx_Impl, ITfThreadMgr, ITfThreadMgrEventSink,
		ITfThreadMgrEventSink_Impl, TF_INVALID_COOKIE,
	},
};
use windows_core::BSTR;

use crate::{
	key_event::Key, my_text_service::MyTextService, prelude::*, text_service,
};

#[implement(
	ITfCompositionSink,
	ITfTextEditSink,
	ITfTextInputProcessorEx,
	ITfThreadMgrEventSink,
	ITfKeyEventSink
)]
#[derive(Default)]
pub struct TextService {
	inner: RwLock<MyTextService>,
	client_id: Cell<u32>,
	thread_mgr_sink_cookie: Cell<u32>,
}

impl TextService {
	fn lock(&self) -> RwLockWriteGuard<'_, MyTextService> {
		self.inner.write().unwrap()
	}
}

impl ITfTextInputProcessor_Impl for TextService_Impl {
	fn Activate(
		&self,
		ptim: WinRef<'_, ITfThreadMgr>,
		tid: u32,
	) -> WinResult<()> {
		let thread_mgr = ptim.ok()?;
		let mut text_service = self.lock();
		let composition_sink = self.as_interface_ref();

		self.client_id.set(tid);
		if text_service
			.activate(thread_mgr, tid, composition_sink)
			.is_err()
		{
			return Err(E_UNEXPECTED.into());
		}
		log::debug!("activate2");

		unsafe {
			let cookie = thread_mgr.cast::<ITfSource>()?.AdviseSink(
				&ITfThreadMgrEventSink::IID,
				self.as_interface_ref(),
			)?;
			self.thread_mgr_sink_cookie.set(cookie);

			thread_mgr.cast::<ITfKeystrokeMgr>()?.AdviseKeyEventSink(
				tid,
				self.as_interface_ref(),
				true,
			)?;
		}
		Ok(())
	}

	fn Deactivate(&self) -> windows_core::Result<()> {
		let mut text_service = self.lock();
		let Some(thread_mgr) = text_service.deactivate() else {
			return Err(E_UNEXPECTED.into());
		};

		let tid = self.client_id.get();

		unsafe {
			thread_mgr.cast::<ITfSource>()?.UnadviseSink(
				self.thread_mgr_sink_cookie.replace(TF_INVALID_COOKIE),
			)?;

			thread_mgr
				.cast::<ITfKeystrokeMgr>()?
				.UnadviseKeyEventSink(tid)?;
		}
		Ok(())
	}
}

impl ITfTextInputProcessorEx_Impl for TextService_Impl {
	fn ActivateEx(
		&self,
		ptim: WinRef<'_, ITfThreadMgr>,
		tid: u32,
		_dwflags: u32,
	) -> WinResult<()> {
		self.Activate(ptim, tid)
	}
}

impl ITfThreadMgrEventSink_Impl for TextService_Impl {
	fn OnInitDocumentMgr(
		&self,
		pdim: WinRef<'_, ITfDocumentMgr>,
	) -> WinResult<()> {
		Ok(())
	}

	fn OnUninitDocumentMgr(
		&self,
		pdim: WinRef<'_, ITfDocumentMgr>,
	) -> WinResult<()> {
		Ok(())
	}

	fn OnPushContext(&self, _pic: WinRef<ITfContext>) -> WinResult<()> {
		Ok(())
	}

	fn OnPopContext(&self, _pic: WinRef<ITfContext>) -> WinResult<()> {
		Ok(())
	}

	fn OnSetFocus(
		&self,
		pdimfocus: WinRef<'_, ITfDocumentMgr>,
		pdimprevfocus: WinRef<'_, ITfDocumentMgr>,
	) -> WinResult<()> {
		let mut text_service = self.lock();
		if pdimfocus.is_null() {
			if let Some(doc_mgr) = pdimprevfocus.as_ref() {
				// PASTING CHEWING COMMENTS:
				// From MSTF doc: To simplify this process and prevent
				// multiple modal UIs from being displayed, there is a maximum
				// of two contexts allowed on the stack.
				//
				// XXX: We don't push contexts, so there should always only one
				// context. It doesn't matter we get the Top or the Base.
				let context = unsafe { doc_mgr.GetBase()? };
				if text_service.kill_focus(&context).is_err() {
					return Err(E_UNEXPECTED.into());
				}
			}
		}
		Ok(())
	}
}

impl ITfTextEditSink_Impl for TextService_Impl {
	fn OnEndEdit(
		&self,
		pic: WinRef<'_, ITfContext>,
		ecreadonly: u32,
		peditrecord: WinRef<'_, ITfEditRecord>,
	) -> WinResult<()> {
		log::debug!("TODO: OnEndEdit");
		Ok(())
	}
}

impl ITfCompositionSink_Impl for TextService_Impl {
	fn OnCompositionTerminated(
		&self,
		ecwrite: u32,
		pcomposition: WinRef<'_, ITfComposition>,
	) -> WinResult<()> {
		self.lock().terminate_composition();
		Ok(())
	}
}

fn to_key(wparam: WPARAM) -> WinResult<Option<Key>> {
	let WPARAM(wparam) = wparam;
	if wparam > u8::MAX as _ {
		return Ok(None);
	}
	let wparam = wparam as u8;

	let mut keyboard_vk_state = [0u8; 256];
	unsafe { GetKeyboardState(&mut keyboard_vk_state)? };

	Ok(Key::from_windows_keycode(wparam, keyboard_vk_state))
}

impl ITfKeyEventSink_Impl for TextService_Impl {
	fn OnSetFocus(&self, fforeground: BOOL) -> WinResult<()> {
		Ok(())
	}

	fn OnTestKeyDown(
		&self,
		pic: WinRef<'_, ITfContext>,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> WinResult<BOOL> {
		// TODO
		log::debug!(
			"TODO: OnTestKeyDown(wparam: 0x{:x}, lparam: 0x{:x})",
			wparam.0,
			lparam.0
		);
		// to_key(wparam)?;
		Ok(false.into())
	}

	fn OnTestKeyUp(
		&self,
		pic: WinRef<'_, ITfContext>,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> WinResult<BOOL> {
		// TODO
		log::debug!(
			"TODO: OnTestKeyUp(wparam: 0x{:x}, lparam: 0x{:x})",
			wparam.0,
			lparam.0
		);
		// to_key(wparam)?;
		Ok(false.into())
	}

	fn OnPreservedKey(
		&self,
		pic: WinRef<'_, ITfContext>,
		rguid: *const GUID,
	) -> WinResult<BOOL> {
		log::debug!("TODO: OnPreservedKey");
		Ok(false.into())
	}

	fn OnKeyDown(
		&self,
		pic: WinRef<'_, ITfContext>,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> WinResult<BOOL> {
		log::debug!(
			"TODO: OnKeyDown(wparam: 0x{:x}, lparam: 0x{:x})",
			wparam.0,
			lparam.0
		);
		Ok(false.into())
	}

	fn OnKeyUp(
		&self,
		pic: WinRef<'_, ITfContext>,
		wparam: WPARAM,
		lparam: LPARAM,
	) -> WinResult<BOOL> {
		log::debug!(
			"TODO: OnKeyUp(wparam: 0x{:x}, lparam: 0x{:x})",
			wparam.0,
			lparam.0
		);
		Ok(false.into())
	}
}
