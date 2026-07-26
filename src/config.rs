// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::filesystem::{ensure_dir, ensure_file, ensure_file_with_content};
use crate::manifest::{load_manifest, save_manifest};
use crate::result::Result;
use crate::paths::Paths;
use crate::templates;



#[derive(Debug, Error)]
pub enum ConfigError {
	#[error("Failed to open albumctl config")]
	OpenConfig,

	#[error("Failed to save albumctl config")]
	SaveConfig
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigManifest {
	default_library: Option<PathBuf>
}

#[derive(Debug)]
pub struct Config {
	manifest: ConfigManifest
}

impl Config {
	pub fn new() -> Result<Config, ConfigError> {
		let paths = Paths::new()
			.change_context(ConfigError::OpenConfig)?;

		ensure_dir(&paths.config_dir)
			.change_context(ConfigError::OpenConfig)?;

		ensure_file_with_content(&paths.config_manifest, Some(templates::CONFIG))
			.change_context(ConfigError::OpenConfig)?;

		let manifest = load_manifest::<ConfigManifest>(&paths.config_manifest)
			.change_context(ConfigError::OpenConfig)?;

		Ok(Config { manifest })
	}

	pub fn save(&self) -> Result<(), ConfigError> {
		let paths = Paths::new()
			.change_context(ConfigError::SaveConfig)?;

		save_manifest(&self.manifest, &paths.config_manifest)
			.change_context(ConfigError::SaveConfig)?;

		Ok(())
	}

	pub fn set_default_library(&mut self, path: Option<&Path>) {
		self.manifest.default_library = path.map(PathBuf::from);
	}
}
