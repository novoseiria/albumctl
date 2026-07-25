// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};
use std::fs::{self, OpenOptions};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;



#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("Failed to ensure directory {path}")]
	EnsureDir { path: PathBuf },

	#[error("Failed to ensure empty directory {path}")]
	EnsureEmptyDir { path: PathBuf },

	#[error("Failed to ensure file {path}")]
	EnsureFile { path: PathBuf }
}

pub fn ensure_dir(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureDir { path: path.to_path_buf() };

	if path.exists() && !path.is_dir() {
		Err(error()).attach_with(
			|| format!("{} exists but is not a directory",path.display())
		)?;
	}

	fs::create_dir_all(path)
		.change_context_lazy(error)
		.attach_with(|| format!("while creating {}", path.display()))?;

	Ok(())
}

pub fn ensure_empty_dir(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureEmptyDir { path: path.to_path_buf() };

	ensure_dir(path)?;

	let mut entries = path.read_dir()
		.change_context_lazy(error)
		.attach_with(|| format!("while reading {}", path.display()))?;

	if let Some(entry) = entries.next() {
		entry.change_context_lazy(error)
			.attach_with(|| format!("while reading {}", path.display()))?;

		Err(error())
			.attach_with(|| format!("{} is not empty", path.display()))?;
	}

	Ok(())
}

pub fn ensure_file(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureFile { path: path.to_path_buf() };

	if path.exists() && !path.is_file() {
		Err(error()).attach_with(
			|| format!("{} exists but is not a file", path.display())
		)?;
	}

	OpenOptions::new()
		.write(true)
		.create(true)
		.open(path)
		.change_context_lazy(error)
		.attach_with(|| format!("while opening {}", path.display()))?;

	Ok(())
}
