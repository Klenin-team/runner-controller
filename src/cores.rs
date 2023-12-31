use tokio::sync::mpsc::Receiver;

mod api;
mod tester;

use crate::structs;

use self::tester::Tester;

pub async fn start_process(core: u8, mut rx: Receiver<structs::Solve>) {
    let mut tester = Tester::new(core).await;

    while let Some(solution) = rx.recv().await {
        // Creating file
        tester.put_code(solution.code, solution.language.filename).await;
        

        if solution.language.compilible {
            // TODO
        } else {
            // Run tests
            for test in solution.tests.iter() {
                println!("{}", tester.run_test(&solution, test).await);
            }
        }

    }
}

