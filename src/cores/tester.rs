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

    pub async fn run_test(&mut self, solution: &Solve, test: &Test) -> Verdicts {
        let mut input_name = solution.input_name;
        let mut output_name = solution.output_name;
        if solution.stdio {
            input_name = "input.txt";
            output_name = "output.txt";
        }
        self.api.create_file(&input_name, test.input).await;
        self.api.create_file(&output_name, test.output).await;


        let mut command = solution.language.execute_command.clone();
        command.push(solution.language.filename);

        let answer = self.api.run(command, Some(solution.stdio), None, None).await;

        if answer["limit_verdict"] == "RealTimeLimitExceeded" {
            return Verdicts::TL;
        } else if answer["limit_verdict"] == "MemoryLimitExceeded" {
            return Verdicts::ML;
        } else if answer["exit_code"] != 0 {
            return Verdicts::RE;
        }
        
        let command_output = self.api.read_file(output_name).await;
        if command_output.trim() != test.output {
            return Verdicts::WA;
        }

        Verdicts::OK
    }
}
