// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use thiserror::Error;

use crate::result::Result;

#[derive(Debug, Error)]
pub enum FilesystemError {
	#[error("{path} does not exist")]
	RequireExists { path: PathBuf },

	#[error("{path} is not a directory")]
	RequireDir { path: PathBuf },

	#[error("{path} is not a file")]
	RequireFile { path: PathBuf },

	#[error("Failed to ensure a directory exists at {path}")]
	EnsureDir { path: PathBuf },

	#[error("Failed to ensure an empty directory exists at {path}")]
	EnsureEmptyDir { path: PathBuf },

	#[error("Failed to ensure a file exists at {path}")]
	EnsureFile { path: PathBuf },

	#[error("Failed to ensure a file with content exists at {path}")]
	EnsureFileWithContent { path: PathBuf },
}

pub fn require_exists(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::RequireExists {
		path: path.to_path_buf(),
	};
	if !path.exists() {
		Err(error())?;
	}

	Ok(())
}

pub fn require_dir(path: &Path) -> Result<(), FilesystemError> {
	require_exists(path)?;

	let error = || FilesystemError::RequireDir {
		path: path.to_path_buf(),
	};
	if !path.is_dir() {
		Err(error())?;
	}

	Ok(())
}

pub fn require_fiile(path: &Path) -> Result<(), FilesystemError> {
	require_exists(path)?;

	let error = || FilesystemError::RequireFile {
		path: path.to_path_buf(),
	};
	if !path.is_file() {
		Err(error())?;
	}

	Ok(())
}

pub fn ensure_dir(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureDir {
		path: path.to_path_buf(),
	};

	if path.exists() && !path.is_dir() {
		Err(error()).attach_with(|| format!("{} exists but is not a directory", path.display()))?;
	}

	fs::create_dir_all(path)
		.change_context_lazy(error)
		.attach_with(|| format!("while creating {}", path.display()))?;

	Ok(())
}

pub fn ensure_empty_dir(path: &Path) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureEmptyDir {
		path: path.to_path_buf(),
	};

	ensure_dir(path)?;

	let mut entries = path
		.read_dir()
		.change_context_lazy(error)
		.attach_with(|| format!("while reading from {}", path.display()))?;

	if let Some(entry) = entries.next() {
		entry
			.change_context_lazy(error)
			.attach_with(|| format!("while reading from {}", path.display()))?;

		Err(error()).attach_with(|| format!("{} is not empty", path.display()))?;
	}

	Ok(())
}

pub fn ensure_file_with_content(path: &Path, content: Option<&str>) -> Result<(), FilesystemError> {
	let error = || FilesystemError::EnsureFile {
		path: path.to_path_buf(),
	};
	let write_error = || FilesystemError::EnsureFileWithContent {
		path: path.to_path_buf(),
	};

	if path.exists() && !path.is_file() {
		Err(error()).attach_with(|| format!("{} exists but is not a file", path.display()))?;
	}

	let mut result = OpenOptions::new().write(true).create_new(true).open(path);

	match &mut result {
		Err(e) if e.kind() != io::ErrorKind::AlreadyExists => {
			result
				.change_context_lazy(error)
				.attach_with(|| format!("while opening {}", path.display()))?;
		}

		Ok(file) if let Some(content) = content => {
			file.write_all(content.as_bytes())
				.change_context_lazy(write_error)
				.attach_with(|| format!("while writing to {}", path.display()))?;
		}

		_ => {}
	}

	Ok(())
}

pub fn ensure_file(path: &Path) -> Result<(), FilesystemError> {
	ensure_file_with_content(path, None)
}
