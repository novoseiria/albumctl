// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{Parser, Subcommand};



#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
	#[command(subcommand)]
	command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command {
	Init {
		path: PathBuf,

		#[arg(short = 'd', long)]
		make_default: bool
	},
	Album {
		path: PathBuf,
		library: Option<PathBuf>
	},
	Release {
		path: PathBuf,
		library: Option<PathBuf>
	}
}
