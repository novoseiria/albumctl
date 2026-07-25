// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

mod album;
mod config;
mod cli;
mod filesystem;
mod library;
mod manifest;
mod release;
mod paths;
mod result;
mod tracklist;

use clap::Parser;

use crate::cli::Cli;



fn main() {
	let args = Cli::parse();

	eprintln!("{args:?}");
}
