// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::result::Result;



#[derive(Debug, Error)]
pub enum ManifestError {
	#[error("Failed to load manifest {path}")]
	LoadManifest { path: PathBuf }
}

pub fn load_manifest<T: DeserializeOwned>(path: &Path) -> Result<T, ManifestError> {
	let error = || ManifestError::LoadManifest { path: path.to_path_buf() };

	let data = fs::read_to_string(&path)
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {}", path.display()))?;

	let manifest = toml::from_str(&data)
		.change_context_lazy(error)
		.attach_with(|| format!("while parsing {}", path.display()))?;

	Ok(manifest)
}
