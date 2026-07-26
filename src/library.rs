// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::config::Config;
use crate::filesystem::{ensure_empty_dir, ensure_file_with_content, require_dir};
use crate::manifest::load_manifest;
use crate::result::Result;
use crate::templates;

#[derive(Debug, Error)]
pub enum LibraryError {
	#[error("Failed to initialize music library at {path}")]
	InitLibrary { path: PathBuf },

	#[error("No default music library is set")]
	NoDefaultLibrary,

	#[error("Failed to open music library at {path}")]
	OpenLibrary { path: PathBuf },
}

#[derive(Debug, Deserialize)]
pub struct LibraryManifest {}

#[derive(Debug)]
pub struct Library {
	manifest: LibraryManifest,
}

impl Library {
	pub fn init(path: &Path, make_default: bool, config: &mut Config) -> Result<(), LibraryError> {
		let error = || LibraryError::InitLibrary {
			path: path.to_path_buf(),
		};
		let manifest_path = path.join("library.toml");

		ensure_empty_dir(path).change_context_lazy(error)?;
		ensure_file_with_content(&manifest_path, Some(templates::LIBRARY))
			.change_context_lazy(error)?;

		if make_default {
			let canonical = path
				.canonicalize()
				.change_context_lazy(error)
				.attach_with(|| format!("while resolving {}", path.display()))?;

			config.set_default_library(Some(&canonical));
			config.save().change_context_lazy(error)?;
		}

		Ok(())
	}

	pub fn open(path: Option<&Path>, config: &Config) -> Result<Library, LibraryError> {
		let path = path
			.or(config.default_library())
			.ok_or(LibraryError::NoDefaultLibrary)?;

		let manifest_path = path.join("library.toml");

		let error = || LibraryError::OpenLibrary {
			path: path.to_path_buf(),
		};
		require_dir(path).change_context_lazy(error)?;

		let manifest = load_manifest(&manifest_path).change_context_lazy(error)?;

		Ok(Library { manifest })
	}
}
