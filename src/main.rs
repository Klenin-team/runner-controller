use std::collections::LinkedList;

use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

mod cores;
mod structs;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<structs::Solve>(1);
    tokio::spawn(async move {
        cores::start_process(3, rx).await;
    });
    let tests = LinkedList::from([
        structs::Test { input: "12\n", output: "12" },
        structs::Test { input: "12\n", output: "13" }
    ]);
    let language = structs::Language {
        filename: "solution.py",
        compilible: false,
        compile_command: vec![],
        execute_command: vec!["/usr/bin/python"]
    };
    let solution = structs::Solve {
        code: "print(input())",
        stdio: true,
        input_name: "",
        output_name: "",
        tests,
        language
    };
    tx.send(solution).await.expect("");
    sleep(Duration::from_millis(15000)).await;
}
