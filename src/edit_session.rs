use std::{cell::OnceCell, mem::ManuallyDrop};

use windows::Win32::UI::TextServices::{
	ITfComposition, ITfCompositionSink, ITfContext, ITfContextComposition,
	ITfEditSession, ITfEditSession_Impl, ITfInsertAtSelection, TF_AE_NONE,
	TF_ANCHOR_END, TF_DEFAULT_SELECTION, TF_IAS_QUERYONLY, TF_SELECTION,
	TF_SELECTIONSTYLE, TF_ST_CORRECTION,
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

		log::debug!("start composition");

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
		let mut selection = [TF_SELECTION::default()];
		let mut fetched = 0;
		let get_selection = unsafe {
			self.context.GetSelection(
				ec,
				TF_DEFAULT_SELECTION,
				&mut selection,
				&mut fetched,
			)
		};
		if get_selection.is_ok() {
			if let Some(range) = &*selection[0].range {
				// unsafe { range.ShiftEndToRange(ec, range, TF_ANCHOR_END) }?;
				unsafe { range.Collapse(ec, TF_ANCHOR_END) }?;
				selection[0].style.fInterimChar = false.into();
				unsafe { self.context.SetSelection(ec, &selection) }?;
			}
		}
		unsafe { self.composition.EndComposition(ec) }?;

		log::debug!("end composition");

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
		let range = unsafe { self.composition.GetRange() }?;
		unsafe { range.SetText(ec, TF_ST_CORRECTION, self.text) }?;

		let selection = TF_SELECTION {
			range: ManuallyDrop::new(Some(range)),
			style: TF_SELECTIONSTYLE {
				ase: TF_AE_NONE,
				fInterimChar: true.into(),
			},
		};

		// Without this, OnCompositionTerminated triggers on notepad.exe
		unsafe { self.context.SetSelection(ec, &[selection]) }?;

		log::debug!("set composition string");

		Ok(())
	}
}
