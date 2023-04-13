#[macro_use]
mod error;
mod latex;
mod transform;

use error::{Error, Result};
use latex::Latex;

use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::process::ExitCode;

fn arg(index: usize) -> Result<String> {
    Ok(env::args().nth(index).ok_or(Error::InsufficientArgs)?)
}

fn try_main() -> Result<()> {
    let filepath = match PathBuf::from(arg(1)?) {
        v if v.is_absolute() => v,
        v => env::current_dir()?.join(v),
    };
    let job_name = filepath.file_stem().ok_or(Error::FilenameNotFound)?;
    let latex = Latex::new(&job_name)?;
    let src_file = File::open(&filepath)?;
    latex.build(&src_file)
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
