// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use directories::ProjectDirs;
use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;



#[derive(Debug, Error)]
pub enum PathsError {
	#[error("Could not resolve albumctl file and directory paths")]
	ResolvePaths
}

#[derive(Debug)]
pub struct Paths {
	pub config_dir: PathBuf,
	pub config_manifest: PathBuf
}

impl Paths {
	pub fn new() -> Result<Paths, PathsError> {
		let project_dirs = ProjectDirs::from("com", "novoseiria", "albumctl")
			.ok_or(PathsError::ResolvePaths)
			.attach("could not retrieve the user's home directory")?;

		let config_dir = project_dirs.config_dir().to_path_buf();
		let config_manifest = config_dir.join("config.toml");

		Ok(Paths { config_dir, config_manifest })
	}
}
