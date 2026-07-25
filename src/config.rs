// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::filesystem::{ensure_dir, ensure_file};
use crate::manifest::load_manifest;
use crate::result::Result;
use crate::paths::Paths;



#[derive(Debug, Error)]
pub enum ConfigError {
	#[error("Failed to open albumctl config")]
	ConfigNew
}

#[derive(Debug, Deserialize)]
pub struct ConfigManifest {
	default_library: PathBuf
}

#[derive(Debug)]
pub struct Config {
	manifest: ConfigManifest
}

impl Config {
	pub fn new() -> Result<Config, ConfigError> {
		let paths = Paths::new()
			.change_context(ConfigError::ConfigNew)?;

		ensure_dir(&paths.config_dir)
			.change_context(ConfigError::ConfigNew)?;

		ensure_file(&paths.config_manifest)
			.change_context(ConfigError::ConfigNew)?;

		let manifest = load_manifest::<ConfigManifest>(&paths.config_manifest)
			.change_context(ConfigError::ConfigNew)?;

		Ok(Config { manifest })
	}
}
