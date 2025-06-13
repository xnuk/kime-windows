use std::{collections::HashMap, env, fs, path::PathBuf};

use kime_engine_backend_hangul::{HangulData, Layout, builtin_layouts};
use kime_engine_core::{Config, EngineConfig};
use serde::Deserialize;

fn config_folder_path() -> PathBuf {
	let mut path =
		PathBuf::from(env::var_os("USERPROFILE").unwrap_or_default());
	path.push(".config");
	path.push("kime");
	path
}

#[derive(Deserialize)]
struct RawConfig {
	engine: Option<EngineConfig>,
}

fn is_yaml_file(entry: &fs::DirEntry) -> bool {
	let is_file = entry.file_type().map(|x| x.is_file()).unwrap_or(false);

	let is_yaml = if let Some(extension) = entry.path().extension() {
		if let Some(extension) = extension.to_str() {
			extension == "yaml"
		} else {
			false
		}
	} else {
		false
	};

	is_file && is_yaml
}

pub fn read_config() -> anyhow::Result<Config> {
	let config_path = config_folder_path().join("config.yaml");
	let file = fs::read_to_string(config_path)?;
	let RawConfig { engine } = serde_yaml::from_str(&file)?;
	if let Some(engine) = engine {
		let hangul_data = {
			let mut layouts = HashMap::new();
			let folder = fs::read_dir(config_folder_path().join("layouts"));
			if let Ok(folder) = folder {
				for entry in folder.flatten() {
					let entry_path = entry.path();
					let key = entry_path
						.file_stem()
						.and_then(|x| x.to_str().map(|x| x.to_string()));

					if let Some(key) = key {
						if is_yaml_file(&entry) {
							if let Some(layout) = fs::read_to_string(entry_path)
								.ok()
								.and_then(|x| serde_yaml::from_str(&x).ok())
							{
								layouts.insert(key, Layout::from_items(layout));
							}
						}
					}
				}
			}

			HangulData::new(
				&engine.hangul,
				builtin_layouts()
					.chain(layouts.into_iter().map(|(k, v)| (k.into(), v))),
			)
		};

		let mut config = Config::new(engine);
		config.hangul_data = hangul_data;
		Ok(config)
	} else {
		Ok(Config::default())
	}
}
