// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod album;
mod config;
mod cli;
mod filesystem;
mod library;
mod manifest;
mod release;
mod run;
mod paths;
mod result;
mod tracklist;

use std::process::ExitCode;

use error_stack::{fmt::ColorMode, Report};

use crate::run::run;



fn main() -> ExitCode {
	Report::set_color_mode(ColorMode::Color);

	if let Err(e) = run() {
		eprintln!("{e:?}");
		ExitCode::FAILURE
	}
	else {
		ExitCode::SUCCESS
	}
}
