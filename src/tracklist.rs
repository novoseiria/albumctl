// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Disc {
	tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
	title: String,
}
