use tokio::sync::{mpsc, oneshot};

mod api;
mod tester;

use crate::structs::{self, Verdicts};

use self::tester::Tester;

pub async fn start_process(core: u8, mut rx: mpsc::Receiver<(structs::Solve, oneshot::Sender<Vec<structs::Verdicts>>)>) {
    let mut tester = Tester::new(core).await;

    while let Some(message) = rx.recv().await {
        let solution = message.0;
        let mut tests: Vec<Verdicts> = Vec::new();
        if solution.language.compilible {
            let (ok, file) = tester.compile(&solution).await;
            if ok == false {
                message.1.send(vec![Verdicts::CE]).expect("Channel closed");
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

