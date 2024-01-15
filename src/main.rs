use std::{collections::HashMap, borrow::Borrow};
use controller::json_to_solution;
use tokio::sync::{ mpsc, oneshot };

use config::Config;
use tokio::time::{sleep, Duration};

mod cores;
mod controller;
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

    let run_on_cores = settings.get::<Vec<u8>>("cores")
        .expect("No cores specified in config file");
    let queue_url = settings.get::<String>("queue_url")
        .expect("No queue specified in config file");
    let verdicts_return_url = settings.get::<String>("verdicts_return_url")
        .expect("No verdicts_return_url specified in config file");
    let queue_poll_interval = settings.get::<u64>("queue_poll_interval").unwrap_or(10);
    let languages = structs::languages::set_languages();

    let mut free_cores = run_on_cores.clone();

    let mut senders: HashMap<u8, mpsc::Sender<(structs::Solve, oneshot::Sender<Vec<structs::Verdict>>)>> = HashMap::new();
    for core in run_on_cores {
        let (tx, rx) = mpsc::channel::<(structs::Solve, oneshot::Sender<Vec<structs::Verdict>>)>(1);
        tokio::spawn(async move {
            cores::start_process(core, rx).await;
        });
        senders.insert(core, tx);
    }

    let total_cores = free_cores.capacity();
    let (freed_core_tx, mut freed_core_rx) = mpsc::channel::<u8>(total_cores);

    loop {
        let mut freed_core = freed_core_rx.try_recv();
        while freed_core.is_ok() {
            free_cores.push(freed_core.unwrap());
            freed_core = freed_core_rx.try_recv()
        }
        while free_cores.is_empty() == false {
            let queue_url = queue_url.clone();
            let res = reqwest::get(queue_url.to_string() + "/solution").await;
            if res.is_err() {
                break; // Queue is down
            }
            let res = res.unwrap();
            if res.status() != 200 {
                // Seems like the queue is down
                break;
            }
            let solution_and_id = json_to_solution(res.text().await.expect("").borrow(), &languages);
            if solution_and_id.is_err() {
                break; // Queue is empty
            }
            let (id, solution) = solution_and_id.unwrap();
            let available_core = free_cores.pop().unwrap();
            controller::run(
                senders.get(&available_core).unwrap(), solution, freed_core_tx.clone(), 
                available_core, verdicts_return_url.clone(), id
            ).await;
        }
        sleep(Duration::from_secs(queue_poll_interval)).await;
    }

}
