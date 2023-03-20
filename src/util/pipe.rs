use std::process::exit;

use anyhow::Result;
use bevy::prelude::*;

pub fn log_errors(In(result): In<Result<()>>) {
	if let Err(error) = result {
		error!("{:?}", error);
		exit(-1);
	}
}
