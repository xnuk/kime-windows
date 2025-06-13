use widestring::U16CString;
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows_core::PCWSTR;

use crate::meta;

struct Logger;
static LOGGER: Logger = Logger;
const PREFIX: &str = meta::NAME;

impl log::Log for Logger {
	fn enabled(&self, metadata: &log::Metadata) -> bool {
		metadata.level() <= log::Level::Debug
	}

	fn log(&self, record: &log::Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let file = record.file().unwrap_or("<unknown>");
		let line = record.line().map(|v| v.to_string()).unwrap_or("?".into());
		let level = record.level();
		let args = record.args();

		let s = U16CString::from_str_truncate(format!(
			"[{PREFIX}] {file}:L{line}:{level}: {args}\r\n"
		));
		unsafe {
			OutputDebugStringW(PCWSTR(s.as_ptr()));
		}
	}

	fn flush(&self) {}
}

pub fn init() {
	if let Err(e) = log::set_logger(&LOGGER) {
		let s = U16CString::from_str_truncate(format!(
			"Failed to setup logger: {e}"
		));
		unsafe {
			OutputDebugStringW(PCWSTR(s.as_ptr()));
		}
		return;
	}

	log::set_max_level(log::LevelFilter::Debug);
}
