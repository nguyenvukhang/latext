#[macro_use]
mod error;
mod latex;
mod transform;

use error::{Error, Result};
use latex::Latex;

use std::fs::File;
use std::process::ExitCode;
use std::{env, path::PathBuf};

fn first_arg() -> Result<String> {
    Ok(env::args().skip(1).next().ok_or(Error::InsufficientArgs)?)
}

fn src_filepath() -> Result<PathBuf> {
    Ok(env::current_dir()?.join(first_arg()?))
}

fn src_file() -> Result<(File, String)> {
    let path = src_filepath()?;
    let file = File::open(&path)?;
    let name = path.file_stem().ok_or(Error::FileStemNotFound)?;
    Ok((file, name.to_string_lossy().to_string()))
}

fn try_main() -> Result<()> {
    let (src_file, job_name) = src_file()?;
    Latex::new(&job_name)?.build(&src_file)?;
    Ok(())
}

fn main() -> ExitCode {
    match try_main() {
        Ok(_) => {
            println!("[successful build!]");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
