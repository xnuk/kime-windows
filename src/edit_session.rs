use std::cell::OnceCell;

use windows::Win32::UI::TextServices::{
	GUID_PROP_ATTRIBUTE, ITfComposition, ITfCompositionSink, ITfContext,
	ITfContextComposition, ITfEditSession, ITfEditSession_Impl,
	ITfInsertAtSelection, TF_ANCHOR_END, TF_DEFAULT_SELECTION,
	TF_IAS_QUERYONLY, TF_SELECTION,
};

use crate::prelude::*;

#[implement(ITfEditSession)]
pub struct StartComposition {
	context: ITfContext,
	composition_sink: ITfCompositionSink,
	pub composition: OnceCell<ITfComposition>,
}

impl StartComposition {
	pub fn new(
		context: ITfContext,
		composition_sink: ITfCompositionSink,
	) -> Self {
		Self {
			context,
			composition_sink,
			composition: OnceCell::new(),
		}
	}
}

impl ITfEditSession_Impl for StartComposition_Impl {
	fn DoEditSession(&self, ec: u32) -> WinResult<()> {
		let context_composition: ITfContextComposition = self.context.cast()?;
		let range = unsafe {
			let selection: ITfInsertAtSelection = self.context.cast()?;
			selection.InsertTextAtSelection(ec, TF_IAS_QUERYONLY, &[])?
		};

		log::debug!("range = {range:?}");

		let composition = unsafe {
			context_composition.StartComposition(
				ec,
				&range,
				&self.composition_sink,
			)
		};

		if let Ok(composition) = composition {
			// TODO: reset the selection
			self.composition.set(composition).ok();
		}

		Ok(())
	}
}

#[implement(ITfEditSession)]
pub struct EndComposition<'a> {
	context: &'a ITfContext,
	composition: &'a ITfComposition,
}

impl<'a> EndComposition<'a> {
	pub(super) fn new(
		context: &'a ITfContext,
		composition: &'a ITfComposition,
	) -> EndComposition<'a> {
		Self {
			context,
			composition,
		}
	}
}

impl ITfEditSession_Impl for EndComposition_Impl<'_> {
	fn DoEditSession(&self, ec: u32) -> WinResult<()> {
		unsafe {
			let range = self.composition.GetRange()?;

			// what is it
			// let prop = self.context.GetProperty(&GUID_PROP_ATTRIBUTE)?;
			// prop.Clear(ec, &range)?;

			let mut selection = [TF_SELECTION::default(); 1];
			let mut selection_len = 0;
			self.context.GetSelection(
				ec,
				TF_DEFAULT_SELECTION,
				&mut selection,
				&mut selection_len,
			)?;

			if let Some(sel_range) = &*selection[0].range {
				// move and clear
				// TODO: need to understand
				sel_range.ShiftEndToRange(ec, &range, TF_ANCHOR_END)?;
				sel_range.Collapse(ec, TF_ANCHOR_END)?;
				self.context.SetSelection(ec, &selection)?;
			}
			self.composition.EndComposition(ec)?;
		}

		Ok(())
	}
}

#[implement(ITfEditSession)]
pub(super) struct SetCompositionString<'a> {
	context: &'a ITfContext,
	composition: &'a ITfComposition,
	text: &'a HSTRING,
}

impl<'a> SetCompositionString<'a> {
	pub(super) fn new(
		context: &'a ITfContext,
		composition: &'a ITfComposition,
		text: &'a HSTRING,
	) -> SetCompositionString<'a> {
		Self {
			context,
			composition,
			text,
		}
	}
}

impl ITfEditSession_Impl for SetCompositionString_Impl<'_> {
	fn DoEditSession(&self, ec: u32) -> WinResult<()> {
		unsafe {
			let range = self.composition.GetRange()?;
			range.SetText(ec, 0, self.text)?;
		}
		Ok(())
	}
}
