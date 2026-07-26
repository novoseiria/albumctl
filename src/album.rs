// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::{Path, PathBuf};

use error_stack::ResultExt;
use serde::Deserialize;
use thiserror::Error;

use crate::filesystem::require_file;
use crate::manifest::load_manifest;
use crate::result::Result;
use crate::tracklist::Disc;

#[derive(Debug, Error)]
pub enum AlbumError {
	#[error("Failed to load album at {path}")]
	LoadAlbum { path: PathBuf },
}

#[derive(Debug, Deserialize)]
pub struct AlbumManifest {
	title: String,
	artist: String,
	year: u16,

	discs: Vec<Disc>,
}

#[derive(Debug)]
pub struct Album {
	manifest: AlbumManifest,
}

impl Album {
	pub fn load(path: &Path) -> Result<Album, AlbumError> {
		let error = || AlbumError::LoadAlbum {
			path: path.to_path_buf(),
		};

		require_file(path).change_context_lazy(error)?;
		let manifest = load_manifest(path).change_context_lazy(error)?;

		Ok(Album { manifest })
	}
}
