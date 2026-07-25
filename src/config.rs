// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use serde::Deserialize;



#[derive(Debug, Deserialize)]
struct ConfigManifest {
	default_library: PathBuf
}
