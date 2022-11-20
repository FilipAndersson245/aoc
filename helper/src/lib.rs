use anyhow::Result;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AOCError {
    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse float error")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("int conversion error")]
    TryFromInt(#[from] std::num::TryFromIntError),
    #[error("IO error")]
    TryFromF(#[from] std::io::Error),
    #[error("nom parse error")]
    NomParse(String),
    #[error("incomplete parse (remainder {remainder:?})")]
    IncompleteParse { remainder: String },
    #[error("no possible solution found")]
    NoSolution,
    #[error("logic error ({0})")]
    Logic(&'static str),
    #[error("incorrect input ({0})")]
    IncorrectInput(&'static str),
}

pub fn get_input(year: usize, day: usize) -> Result<String, AOCError> {
    let mut f = File::open(&format!("~/.aoc/{}/{}.txt", year, day))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}
