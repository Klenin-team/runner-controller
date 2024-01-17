use tokio::sync::{mpsc, oneshot};

mod api;
mod tester;

use crate::structs;

use self::tester::Tester;

pub async fn start_process(core: u8, mut rx: mpsc::Receiver<(structs::Solve, oneshot::Sender<Vec<structs::Verdict>>)>) {
    let mut tester = Tester::new(core).await;

    while let Some(message) = rx.recv().await {
        let solution = message.0;
        let mut tests: Vec<structs::Verdict> = Vec::new();
        if solution.language.compilible {
            let (ok, file) = tester.compile(&solution).await;
            if ok == false {
                message.1.send(vec![structs::Verdict { 
                    compilation_output: String::from_utf8(file).unwrap_or("".to_string()), 
                    program_output: "".to_string(),
                    used_time: 0.0,
                    used_memory: 0,
                    verdict: structs::Verdicts::CE }]).expect("Channel closed");
                continue;
            }

            // Run tests
            for test in solution.tests.iter() {
                tests.push(tester.run_test(&solution, test, Some(file.clone())).await);
            }
        } else {
            // Run tests
            for test in solution.tests.iter() {
                tests.push(tester.run_test(&solution, test, None).await);
            }
        }
        message.1.send(tests).expect("Channel closed");

    }
}

