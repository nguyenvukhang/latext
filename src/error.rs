use std::fmt;
use std::io;
use std::io::BufRead;
use std::process::Output;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    InsufficientArgs,
    FileStemNotFound,
    PdfLatexErr(Output),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InsufficientArgs => {
                write!(f, "Please provide a .tex file to build.")
            }
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
            e => write!(f, "{e:?}"),
        }
    }
}

#[rustfmt::skip]
impl From<io::Error>for Error{fn from(v:io::Error)->Self{Self::IoError(v)}}
