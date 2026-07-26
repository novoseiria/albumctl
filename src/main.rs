// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod album;
mod cli;
mod config;
mod filesystem;
mod library;
mod manifest;
mod paths;
mod release;
mod result;
mod run;
mod templates;
mod tracklist;

use std::process::ExitCode;

use error_stack::{Report, fmt::ColorMode};

use crate::run::run;

fn main() -> ExitCode {
	Report::set_color_mode(ColorMode::Color);

	if let Err(e) = run() {
		eprintln!("{e:?}");
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}
