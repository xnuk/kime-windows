use kime_engine_core::{Config, InputEngine, InputResult};
use widestring::U16CString;
use windows::Win32::UI::{
	TextServices::{
		ITfComposition, ITfCompositionSink, ITfContext, ITfThreadMgr,
		TF_ES_READWRITE, TF_ES_SYNC,
	},
	WindowsAndMessaging::{MB_OK, MessageBoxW},
};
use windows_core::{InterfaceRef, PCWSTR, w};

use crate::{
	config::read_config,
	edit_session::{EndComposition, SetCompositionString, StartComposition},
	key_event::Key,
	prelude::*,
};

pub struct MyTextService {
	thread_mgr: Option<ITfThreadMgr>,
	client_id: u32,
	composition: Option<ITfComposition>,
	composition_sink: Option<ITfCompositionSink>,

	input_engine: InputEngine,
	config: Config,
}

impl MyTextService {
	pub fn new() -> Self {
		let config = match read_config() {
			Ok(config) => config,
			Err(err) => {
				let message = U16CString::from_str_truncate(err.to_string());
				unsafe {
					MessageBoxW(
						None,
						PCWSTR(message.as_ptr()),
						w!("Warning"),
						MB_OK,
					)
				};
				Config::default()
			}
		};

		let engine = InputEngine::new(&config);

		Self {
			thread_mgr: None,
			client_id: 0,
			composition: None,
			composition_sink: None,
			input_engine: engine,
			config,
		}
	}

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
		self.input_engine.reset();
		self.thread_mgr.take()
	}

	pub fn kill_focus(&mut self, context: &ITfContext) -> WinResult<()> {
		log::debug!("TODO: kill_focus");
		self.input_engine.reset();
		if self.composition.is_some() {
			self.end_composition(context)?;
		}

		Ok(())
	}

	pub fn terminate_composition(&mut self) {
		log::debug!("TODO: terminate_composition");
		self.input_engine.reset();
	}

	pub fn will_handle_keydown(&self, key: Key) -> bool {
		log::debug!("TODO: will_handle_keydown {key:?}");
		self.input_engine.will_handle_key(key.0, &self.config)
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
		let result = self.input_engine.press_key(key.0, &self.config);

		if result.contains(InputResult::HAS_COMMIT) {
			let text = self.input_engine.commit_str().to_string();
			self.set_composition_string(context, &text)?;
			self.end_composition(context)?;
			self.input_engine.clear_commit();
		}

		if result.contains(InputResult::HAS_PREEDIT) {
			let text = self.input_engine.preedit_str().to_string();
			if self.composition.is_none() {
				self.start_composition(context)?;
			}
			self.set_composition_string(context, &text)?;
		} else {
			self.set_composition_string(context, "")?;
			self.end_composition(context)?;
		}

		Ok(result.contains(InputResult::CONSUMED))
	}

	pub fn keyup(&mut self, context: &ITfContext, key: Key) -> WinResult<bool> {
		log::debug!("TODO: keyup {key:?}");
		Ok(false)
	}

	fn start_composition(&mut self, context: &ITfContext) -> WinResult<()> {
		let Some(sink) = &self.composition_sink else {
			return Ok(());
		};
		let session =
			StartComposition::new(context.clone(), sink.clone()).into_object();
		unsafe {
			context
				.RequestEditSession(
					self.client_id,
					session.as_interface(),
					TF_ES_SYNC | TF_ES_READWRITE,
				)?
				.ok()?;
		}

		self.composition = session.composition.get().cloned();

		Ok(())
	}

	fn end_composition(&mut self, context: &ITfContext) -> WinResult<()> {
		let Some(composition) = &self.composition else {
			return Ok(());
		};
		let session = EndComposition::new(context, composition).into_object();
		unsafe {
			context
				.RequestEditSession(
					self.client_id,
					session.as_interface(),
					TF_ES_SYNC | TF_ES_READWRITE,
				)?
				.ok()?;
		}
		drop(session);
		self.composition = None;

		Ok(())
	}

	fn set_composition_string(
		&mut self,
		context: &ITfContext,
		text: &str,
	) -> WinResult<()> {
		let Some(composition) = &self.composition else {
			return Ok(());
		};
		let text = HSTRING::from(text);
		let session = SetCompositionString::new(context, composition, &text)
			.into_object();
		unsafe {
			let _ = context.RequestEditSession(
				self.client_id,
				session.as_interface(),
				TF_ES_SYNC | TF_ES_READWRITE,
			);
		}
		Ok(())
	}
}
