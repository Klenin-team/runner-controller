use std::collections::LinkedList;

pub mod languages;

#[derive(Clone)]
pub struct Language {
    pub filename: &'static str,
    pub compilible: bool,
    pub compile_command: Vec<&'static str>,
    pub execute_command: Vec<&'static str>
}

pub struct Test {
    pub input: String,
    pub output: String
}

pub struct Solve {
    pub code: String,
    pub stdio: bool,
    pub input_name: String,
    pub output_name: String,
    pub tests: LinkedList<Test>,
    pub language: Language
}

#[derive(Debug)]
pub struct Verdict {
    pub used_memory: u64,
    pub used_time: u32,
    pub verdict: Verdicts,
}

#[derive(Debug)]
pub enum Verdicts {
    OK,
    RE,
    TL,
    ML,
    WA,
    CE,
    SE
}

impl std::fmt::Display for Verdicts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Verdicts::OK => write!(f, "OK"),
            Verdicts::RE => write!(f, "RE"),
            Verdicts::TL => write!(f, "TL"),
            Verdicts::ML => write!(f, "ML"),
            Verdicts::WA => write!(f, "WA"),
            Verdicts::CE => write!(f, "CE"),
            Verdicts::SE => write!(f, "SE"),
        }
    }
}


