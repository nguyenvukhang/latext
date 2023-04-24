use crate::error::{Error, Result};
use crate::transform::{replace, transform};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct Latex {
    child: Child,
    stdin: ChildStdin,
}

const BUILD_DIR: &str = ".build";

impl Latex {
    pub fn new<S: AsRef<OsStr>>(jobname: S) -> Result<Self> {
        let mut c = Command::new("pdflatex");
        c.arg("--halt-on-error");
        c.args(["--output-directory", BUILD_DIR]);
        c.arg("--jobname");
        c.arg(jobname);
        c.stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped());
        let mut child = c.spawn()?;
        // safety of this unwrap is guaranteed after successfully spawning
        let stdin = child.stdin.take().unwrap();
        Ok(Self { child, stdin })
    }

    pub fn build(mut self, file: &File) -> Result<()> {
        fs::create_dir_all(BUILD_DIR)?;
        BufReader::new(file)
            .lines()
            .filter_map(|v| v.ok())
            .map(|v| transform(&v).unwrap_or(v))
            // .map(|v| replace(&v))
            .filter(|v| !v.trim().is_empty())
            .for_each(|v| {
                writeln!(self.stdin, "{v}").ok();
            });
        let output = self.child.wait_with_output()?;
        match output.status.success() {
            true => Ok(()),
            _ => Err(Error::PdfLatexErr(output)),
        }
    }
}
