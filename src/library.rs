// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::config::Config;
use crate::filesystem::{ensure_empty_dir, ensure_file_with_content};
use crate::result::Result;
use crate::templates;



#[derive(Debug, Error)]
pub enum LibraryError {
	#[error("Failed to initialize music library at {path}")]
	InitLibrary { path: PathBuf }
}

#[derive(Debug, Deserialize)]
pub struct LibraryManifest {

}

#[derive(Debug)]
pub struct Library {
	manifest: LibraryManifest
}

impl Library {
	pub fn init(path: &Path, make_default: bool, config: &mut Config)
		-> Result<(), LibraryError> {
		let error = || LibraryError::InitLibrary { path: path.to_path_buf() };
		let manifest_path = path.join("library.toml");

		ensure_empty_dir(path).change_context_lazy(error)?;
		ensure_file_with_content(&manifest_path, Some(templates::LIBRARY))
			.change_context_lazy(error)?;

		if make_default {
			let canonical = path.canonicalize()
				.change_context_lazy(error)
				.attach_with(|| format!("while resolving {}", path.display()))?;

			config.set_default_library(Some(&canonical));
			config.save().change_context_lazy(error)?;
		}

		Ok(())
	}
}
