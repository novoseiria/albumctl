// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

use crate::tracklist::Disc;

#[derive(Debug, Deserialize)]
pub struct AlbumManifest {
	title: String,
	artist: String,
	year: u16,

	discs: Vec<Disc>,
}
