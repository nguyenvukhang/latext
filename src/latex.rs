use crate::error::{Error, Result};
use crate::transform;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStdin};
use std::process::{Command, Stdio};

pub struct Latex {
    child: Child,
    stdin: ChildStdin,
}

const BUILD_DIR: &str = ".build";

impl Latex {
    pub fn new(jobname: &str) -> Result<Self> {
        let mut c = Command::new("pdflatex");
        fs::create_dir_all(BUILD_DIR)?;
        c.arg("--halt-on-error");
        c.args(["--output-directory", BUILD_DIR]);
        c.args(["--jobname", jobname]);
        c.stdin(Stdio::piped()).stderr(Stdio::piped()).stdout(Stdio::piped());
        let mut child = c.spawn()?;
        let stdin = child.stdin.take().unwrap();
        Ok(Self { child, stdin })
    }

    pub fn build(mut self, file: &File) -> Result<()> {
        // safety of this unwrap is guaranteed after piping it
        let reader = BufReader::new(file);
        for line in reader.lines().filter_map(|v| v.ok()) {
            let line = transform::transform(&line).unwrap_or(line);
            let line = line.trim();
            writeln!(self.stdin, "{line}")?;
        }
        let output = self.child.wait_with_output()?;
        match output.status.success() {
            true => Ok(()),
            _ => Err(Error::PdfLatexErr(output)),
        }
    }
}
