// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
	Init {
		path: PathBuf,

		#[arg(short = 'd', long)]
		make_default: bool,
	},
	Album {
		#[command(subcommand)]
		command: AlbumCommand,
	},
	Release {
		#[command(subcommand)]
		command: ReleaseCommand,
	},
}

#[derive(Debug, Subcommand)]
pub enum AlbumCommand {
	Add {
		path: PathBuf,
		library: Option<PathBuf>,
	},
}

#[derive(Debug, Subcommand)]
pub enum ReleaseCommand {
	Add {
		path: PathBuf,
		library: Option<PathBuf>,
	},
}
