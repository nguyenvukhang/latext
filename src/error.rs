use std::fmt;
use std::io;
use std::io::BufRead;
use std::process::Output;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    InsufficientArgs,
    FilenameNotFound,
    PdfLatexErr(Output),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut w = |x: &str| write!(f, "{x}");
        use Error::*;
        match self {
            FilenameNotFound => w("Unable to extract filename."),
            InsufficientArgs => w("Please provide a .tex file to build."),
            PdfLatexErr(v) => {
                let lines = v
                    .stdout
                    .lines()
                    .filter_map(|v| v.ok())
                    .skip_while(|v| !v.starts_with("*!"))
                    .collect::<Vec<String>>();
                for i in 0..lines.len() {
                    match i + 1 < lines.len() {
                        true => writeln!(f, "{}", lines[i])?,
                        false => write!(f, "{}", lines[i])?,
                    }
                }
                Ok(())
            }
            IoError(e) => write!(f, "{e:?}"),
        }
    }
}

#[rustfmt::skip]
impl From<io::Error>for Error{fn from(v:io::Error)->Self{Self::IoError(v)}}
