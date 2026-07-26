// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

use crate::tracklist::Disc;

#[derive(Debug, Deserialize)]
pub struct ReleaseManifest {
	title: String,
	artist: String,
	year: u16,

	catalog_number: String,
	media_type: String,
	audio_channels: String,
	provenance: String,

	parent: ParentAlbum,

	discs: Vec<Disc>,
}

#[derive(Debug, Deserialize)]
pub struct ParentAlbum {
	title: String,
	artist: String,
	year: u16,
}
