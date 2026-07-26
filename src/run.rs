// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use error_stack::ResultExt;
use thiserror::Error;

use crate::album::Album;
use crate::cli::{AlbumCommand, Cli, Command};
use crate::config::Config;
use crate::library::Library;
use crate::result::Result;

#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;

pub fn run() -> Result<(), RunError> {
	let args = Cli::parse();

	let mut config = Config::init().change_context(RunError)?;

	match args.command {
		Command::Init { path, make_default } => {
			Library::init(&path, make_default, &mut config).change_context(RunError)?;
		}

		Command::Album {
			command: AlbumCommand::Add { path, library },
		} => {
			let library = Library::open(library.as_deref(), &config).change_context(RunError)?;
			let album = Album::load(&path).change_context(RunError)?;
			library.add_album(&album).change_context(RunError)?;
		}

		_ => todo!(),
	}

	Ok(())
}
