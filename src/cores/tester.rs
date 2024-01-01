use crate::structs::{Solve, Verdicts, Test};

use super::api::Api;

pub struct Tester {
    api: Api
}

impl Tester {
    pub async fn new(core: u8) -> Self {
        Self {
            api: Api::new(core).await
        }
    }

    pub async fn put_code(&mut self, code: &str, to: &str) {
        self.api.create_file(to, code).await;
    }

    async fn put_compiled_code(&mut self, code: Vec<u8>) {
        self.api.create_raw_file("a.out", code, Some(0o777)).await
    }
    

    pub async fn run_test(&mut self, solution: &Solve, test: &Test, compiled: Option<Vec<u8>>) -> Verdicts {
        let mut input_name = solution.input_name;
        let mut output_name = solution.output_name;
        if solution.stdio {
            input_name = "input.txt";
            output_name = "output.txt";
        }
        self.api.create_file(&input_name, test.input).await;
        self.api.create_file(&output_name, test.output).await;

        if compiled.is_some() {
            self.put_compiled_code(compiled.unwrap()).await;
        } else {
            self.put_code(solution.code, solution.language.filename).await;
        }

        let command = solution.language.execute_command.clone();

        let answer = self.api.run(command, Some(solution.stdio), None, None).await;
        let command_output = self.api.read_file(output_name).await;
        self.api.reset().await;

        if answer["limit_verdict"] == "RealTimeLimitExceeded" {
            return Verdicts::TL;
        } else if answer["limit_verdict"] == "MemoryLimitExceeded" {
            return Verdicts::ML;
        } else if answer["exit_code"] != 0 {
            return Verdicts::RE;
        }
        
        if command_output.trim() != test.output {
            return Verdicts::WA;
        }

        Verdicts::OK
    }

    pub async fn compile(&mut self, solution: &Solve) -> (bool, Vec<u8>) {
        self.put_code(solution.code, solution.language.filename).await;
        self.api.create_file("input.txt", "").await;
        let command = solution.language.compile_command.clone();

        let status = self.api.run(command, Some(true), None, None).await;
        let state = status["exit_code"] == 0;
        let output: Vec<u8>;
        if state {
            output = self.api.raw_read_file("a.out").await;
        } else if self.api.check_for_file("errput.txt").await {
            output = self.api.raw_read_file("errput.txt").await;
        } else if self.api.check_for_file("output.txt").await {
            output = self.api.raw_read_file("output.txt").await;
        } else {
            output = vec![];
        }
        self.api.reset().await;

        (state, output)
    }
}
