use std::{collections::{ LinkedList, HashMap }, borrow::Borrow};
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
    let queue_base_url = settings.get::<&'static str>("queue_base_url").expect("No queue specified in config file");
    let queue_poll_interval = settings.get::<u64>("queue_base_url").expect("No queue poll interval specified in config file");
    let languages = structs::languages::set_languages();

    let free_cores = run_on_cores.clone();

    let mut senders: HashMap<u8, mpsc::Sender<(structs::Solve, oneshot::Sender<Vec<structs::Verdicts>>)>> = HashMap::new();
    for core in run_on_cores {
        let (tx, rx) = mpsc::channel::<(structs::Solve, oneshot::Sender<Vec<structs::Verdicts>>)>(1);
        tokio::spawn(async move {
            cores::start_process(core, rx).await;
        });
        senders.insert(core, tx);
    }

    loop {
        while free_cores.is_empty() == false {
            let res = reqwest::get(queue_base_url.to_string() + "/submission").await.expect("Seems like no internet");
            let json = json::parse(res.text().await.expect("").borrow()).expect("");
            if json["any"] == false {
                break;
            }
            println!("{:?}", json);
            // TODO: run tasks

        }
        sleep(Duration::from_secs(queue_poll_interval)).await;
    }
}
