use tokio::sync::mpsc::Receiver;

mod api;
mod tester;

use crate::structs;

use self::tester::Tester;

pub async fn start_process(core: u8, mut rx: Receiver<structs::Solve>) {
    let mut tester = Tester::new(core).await;

    while let Some(solution) = rx.recv().await {
        if solution.language.compilible {
            let (ok, file) = tester.compile(&solution).await;
            if ok == false {
                // TODO: Return CE
                println!("CE");
                continue;
            }

            // Run tests
            for test in solution.tests.iter() {
                println!("{}", tester.run_test(&solution, test, Some(file.clone())).await);
            }
        } else {
            // Run tests
            for test in solution.tests.iter() {
                println!("{}", tester.run_test(&solution, test, None).await);
            }
        }

    }
}

