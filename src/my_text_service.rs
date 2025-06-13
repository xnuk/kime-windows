use windows::Win32::UI::TextServices::{
	ITfCompositionSink, ITfContext, ITfThreadMgr,
};
use windows_core::InterfaceRef;

use crate::{key_event::Key, prelude::*};

#[derive(Default)]
pub struct MyTextService {
	thread_mgr: Option<ITfThreadMgr>,
	client_id: u32,
	composition_sink: Option<ITfCompositionSink>,
}

impl MyTextService {
	pub fn activate(
		&mut self,
		thread_mgr: &ITfThreadMgr,
		client_id: u32,
		composition_sink: InterfaceRef<ITfCompositionSink>,
	) -> WinResult<()> {
		self.thread_mgr = Some(thread_mgr.clone());
		self.client_id = client_id;
		self.composition_sink = Some(composition_sink.to_owned());
		Ok(())
	}

	pub fn deactivate(&mut self) -> Option<ITfThreadMgr> {
		self.thread_mgr.take()
	}

	pub fn kill_focus(&mut self, context: &ITfContext) -> WinResult<()> {
		log::debug!("TODO: kill_focus");

		Ok(())
	}

	pub fn terminate_composition(&mut self) {
		log::debug!("TODO: terminate_composition");
	}

	pub fn will_handle_keydown(&self, key: Key) -> bool {
		log::debug!("TODO: will_handle_keydown {key:?}");
		false
	}

	pub fn will_handle_keyup(&self, key: Key) -> bool {
		log::debug!("TODO: will_handle_keyup {key:?}");
		false
	}

	pub fn keydown(
		&mut self,
		context: &ITfContext,
		key: Key,
	) -> WinResult<bool> {
		log::debug!("TODO: keydown {key:?}");
		Ok(false)
	}

	pub fn keyup(&mut self, context: &ITfContext, key: Key) -> WinResult<bool> {
		log::debug!("TODO: keyup {key:?}");
		Ok(false)
	}
}
