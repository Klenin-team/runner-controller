use std::collections::LinkedList;

pub struct Language {
    pub filename: &'static str,
    pub compilible: bool,
    pub compile_command: Vec<&'static str>,
    pub execute_command: Vec<&'static str>
}

pub struct Test {
    pub input: &'static str,
    pub output: &'static str
}

pub struct Solve {
    pub code: &'static str,
    pub stdio: bool,
    pub input_name: &'static str,
    pub output_name: &'static str,
    pub tests: LinkedList<Test>,
    pub language: Language
}

pub enum Verdicts {
    OK,
    RE,
    TL,
    ML,
    WA,
    CE
}
