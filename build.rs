#![cfg(windows)]

fn main() {
	windres::Build::new().compile("resource.rc").unwrap();
}
