use std::collections::{ LinkedList, HashMap };
use tokio::sync::{ mpsc, oneshot };

use config::Config;
use tokio::time::{sleep, Duration};

mod cores;
mod structs;

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let run_on_cores = settings.get::<Vec<u8>>("cores").expect("No cores specified in config file");
    let languages = structs::languages::set_languages();


    let mut senders: HashMap<u8, mpsc::Sender<(structs::Solve, oneshot::Sender<Vec<structs::Verdicts>>)>> = HashMap::new();
    for core in run_on_cores {
        let (tx, rx) = mpsc::channel::<(structs::Solve, oneshot::Sender<Vec<structs::Verdicts>>)>(1);
        tokio::spawn(async move {
            cores::start_process(core, rx).await;
        });
        senders.insert(core, tx);

    }

    // Python
    let (resp_tx, resp_rx) = oneshot::channel();
    let tests = LinkedList::from([
        structs::Test { input: "12", output: "12" },
        structs::Test { input: "12", output: "13" }
    ]);
    let language = (languages.get("python").expect("no python((((")).clone();
    let solution = structs::Solve {
        code: "print(input())",
        stdio: true,
        input_name: "",
        output_name: "",
        tests,
        language
    };
    senders.get(&3).expect("hey!").send((solution, resp_tx)).await.expect("");
    let verdicts = resp_rx.await;
    println!("{:?}", verdicts);

    // Compile error c
    let (resp_tx, resp_rx) = oneshot::channel();
    let tests = LinkedList::from([
        structs::Test { input: "12", output: "12" },
        structs::Test { input: "12", output: "13" }
    ]);
    let language = (languages.get("gcc").expect("no python((((")).clone();
    let solution = structs::Solve {
        code: "print(input())",
        stdio: true,
        input_name: "",
        output_name: "",
        tests,
        language
    };
    senders.get(&3).expect("hey!").send((solution, resp_tx)).await.expect("");
    let verdicts = resp_rx.await;
    println!("{:?}", verdicts);

    // C
    let (resp_tx, resp_rx) = oneshot::channel();
    let tests = LinkedList::from([
        structs::Test { input: "12", output: "12" },
        structs::Test { input: "12", output: "13" }
    ]);
    let language = (languages.get("gcc").expect("no python((((")).clone();
    let solution = structs::Solve {
        code: "#include <stdio.h>
int main() {
    int a;
    scanf(\"%d\", &a);
    printf(\"%d\", a);
    return 0;
}",
        stdio: true,
        input_name: "",
        output_name: "",
        tests,
        language
    };
    senders.get(&3).expect("hey!").send((solution, resp_tx)).await.expect("");
    let verdicts = resp_rx.await;
    println!("{:?}", verdicts);


    sleep(Duration::from_millis(15000)).await;
}
