// TODO: bring stuff from Kime

use crate::prelude::{LPARAM, WPARAM};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Key {
	pub code: u16,
	pub state: u8,
}

impl Key {
	// wparam returns virtual key codes: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
	// while lparam returns scan codes: https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#keystroke-message-flags
	// we will use virtual key codes.
	pub fn from_windows_keycode(
		wparam: u8,
		keyboard_vk_state: [u8; 256],
	) -> Option<Self> {
		// TODO
		Some(Self { code: 0, state: 0 })
	}
}
