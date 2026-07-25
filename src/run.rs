// SPDX-FileCopyrightText: Copyright (C) Nile Jocson <novoseiria@gmail.com>
// SPDX-License-Identifier: MPL-2.0

use clap::Parser;
use error_stack::ResultExt;
use thiserror::Error;

use crate::cli::{Cli, Command};
use crate::config::Config;
use crate::result::Result;



#[derive(Debug, Error)]
#[error("albumctl encountered an error")]
pub struct RunError;

pub fn run() -> Result<(), RunError> {
	let args = Cli::parse();

	let config = Config::new()
		.change_context(RunError)?;

	Ok(())
}
