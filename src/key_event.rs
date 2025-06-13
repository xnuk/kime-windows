use kime_engine_core as kime;
use windows::Win32::UI::Input::KeyboardAndMouse::{
	self, VK_CONTROL, VK_LWIN, VK_MENU, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD9,
	VK_RWIN, VK_SHIFT,
};

macro_rules! vk_to_keycode_map {
	($(($vk:ident, $key:ident)),+ $(,)?) => {{
		let mut lookup = [None; 256];
		$(
			lookup[KeyboardAndMouse::$vk.0 as usize] = Some(kime::KeyCode::$key)
		);+;
		lookup
	}};
}

const VK_TO_KEYCODE_MAP: [Option<kime::KeyCode>; 256] = vk_to_keycode_map![
	(VK_A, A),
	(VK_B, B),
	(VK_C, C),
	(VK_D, D),
	(VK_E, E),
	(VK_F, F),
	(VK_G, G),
	(VK_H, H),
	(VK_I, I),
	(VK_J, J),
	(VK_K, K),
	(VK_L, L),
	(VK_M, M),
	(VK_N, N),
	(VK_O, O),
	(VK_P, P),
	(VK_Q, Q),
	(VK_R, R),
	(VK_S, S),
	(VK_T, T),
	(VK_U, U),
	(VK_V, V),
	(VK_W, W),
	(VK_X, X),
	(VK_Y, Y),
	(VK_Z, Z),
	(VK_NUMPAD0, NumZero),
	(VK_NUMPAD1, NumOne),
	(VK_NUMPAD2, NumTwo),
	(VK_NUMPAD3, NumThree),
	(VK_NUMPAD4, NumFour),
	(VK_NUMPAD5, NumFive),
	(VK_NUMPAD6, NumSix),
	(VK_NUMPAD7, NumSeven),
	(VK_NUMPAD8, NumEight),
	(VK_NUMPAD9, NumNine),
	(VK_0, Zero),
	(VK_1, One),
	(VK_2, Two),
	(VK_3, Three),
	(VK_4, Four),
	(VK_5, Five),
	(VK_6, Six),
	(VK_7, Seven),
	(VK_8, Eight),
	(VK_9, Nine),
	(VK_OEM_MINUS, Minus),
	(VK_OEM_PLUS, Equal),
	(VK_OEM_4, OpenBracket),
	(VK_OEM_6, CloseBracket),
	(VK_SHIFT, Shift),
	(VK_LSHIFT, Shift),
	(VK_RSHIFT, Shift),
	(VK_OEM_5, Backslash),
	(VK_OEM_2, Slash),
	(VK_OEM_1, SemiColon),
	(VK_OEM_7, Quote),
	(VK_OEM_3, Grave),
	(VK_OEM_COMMA, Comma),
	(VK_OEM_PERIOD, Period),
	//
	(VK_BACK, Backspace),
	(VK_SPACE, Space),
	(VK_RETURN, Enter),
	(VK_TAB, Tab),
	(VK_CONTROL, ControlL),
	(VK_LCONTROL, ControlL),
	(VK_RCONTROL, ControlR),
	(VK_INSERT, Insert),
	(VK_DELETE, Delete),
	(VK_HOME, Home),
	(VK_END, End),
	(VK_PRIOR, PageUp),
	(VK_NEXT, PageDown),
	//
	(VK_ESCAPE, Esc),
	(VK_CONVERT, Henkan),
	(VK_NONCONVERT, Muhenkan),
	(VK_MENU, AltL),
	(VK_LMENU, AltL),
	(VK_RMENU, AltR),
	(VK_HANGUL, Hangul),
	(VK_HANJA, HangulHanja),
	//
	(VK_LEFT, Left),
	(VK_RIGHT, Right),
	(VK_UP, Up),
	(VK_DOWN, Down),
	//
	(VK_F1, F1),
	(VK_F2, F2),
	(VK_F3, F3),
	(VK_F4, F4),
	(VK_F5, F5),
	(VK_F6, F6),
	(VK_F7, F7),
	(VK_F8, F8),
	(VK_F9, F9),
	(VK_F10, F10),
	(VK_F11, F11),
	(VK_F12, F12),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Key(pub kime::Key);

impl Key {
	// wparam returns virtual key codes: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
	// while lparam returns scan codes: https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#keystroke-message-flags
	// we will use virtual key codes.
	pub fn from_windows_keycode(
		wparam: u8,
		keyboard_vk_state: [u8; 256],
	) -> Option<Self> {
		// TODO

		let numlock = keyboard_vk_state[VK_NUMLOCK.0 as usize] & 0x01 > 0;
		let is_numpad =
			(VK_NUMPAD0.0..=VK_NUMPAD9.0).contains(&(wparam as u16));
		if is_numpad && !numlock {
			return None;
		}

		let Some(code) = VK_TO_KEYCODE_MAP[wparam as usize] else {
			log::debug!("could not find keycode for 0x{:x}", wparam);
			return None;
		};

		let mut state = kime::ModifierState::empty();
		if keyboard_vk_state[VK_SHIFT.0 as usize] & 0x80 > 0 {
			state |= kime::ModifierState::SHIFT;
		}
		if keyboard_vk_state[VK_CONTROL.0 as usize] & 0x80 > 0 {
			state |= kime::ModifierState::CONTROL;
		}
		if keyboard_vk_state[VK_LWIN.0 as usize] & 0x80 > 0
			|| keyboard_vk_state[VK_RWIN.0 as usize] & 0x80 > 0
		{
			state |= kime::ModifierState::SUPER;
		}
		if keyboard_vk_state[VK_MENU.0 as usize] & 0x80 > 0 {
			state |= kime::ModifierState::ALT;
		}

		Some(Self(kime::Key::new(code, state)))
	}
}
