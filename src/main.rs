use std::{collections::{ LinkedList, HashMap }, borrow::Borrow, iter::zip};
use json::JsonValue;
use tokio::sync::{ mpsc, oneshot };

use config::Config;
use tokio::time::{sleep, Duration};

mod cores;
mod structs;
mod queue_parser;

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
    let queue_base_url = settings.get::<String>("queue_base_url").expect("No queue specified in config file");
    let queue_poll_interval = settings.get::<u64>("queue_poll_interval").expect("No queue poll interval specified in config file");
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
            let res = reqwest::get(queue_base_url.to_string() + "/solution").await.expect("Seems like no internet");
            if res.status() != 200 {
                // TODO: return ServerError
                break;
            }
            let json = json::parse(res.text().await.expect("").borrow()).expect("");
            if json["any"] == false {
                break;
            }

            let id = json["id"].to_string();
            let id = id.parse::<u128>();
            if id.is_err() {
                break; // We do not have the proper ID
                       // Probably, the queue is empty
            }
            let id = id.unwrap();

            let code = json["code"].to_string();

            let language = json["language"].to_string();
            let language_as_struct = languages.get(language.as_str());
            if language_as_struct.is_none() {
                // TODO: return ServerError
                break;
            }
            let language_as_struct = language_as_struct.unwrap();
            
            let use_stdio = json["stdio"].as_bool().unwrap_or(true);
            let input_file = json["input_file"].to_string();
            let output_file = json["output_file"].to_string();

            
            let mut tests_list: LinkedList<structs::Test> = LinkedList::new();
            for json_test in json["tests"].members() {
                tests_list.push_back(structs::Test { input: json_test[0].to_string(), output: json_test[0].to_string() })
            }

            let solution = structs::Solve{
                code,
                stdio: use_stdio,
                input_name: input_file,
                output_name: output_file,
                tests: tests_list,
                language: language_as_struct.clone()
            };
            
        }
        sleep(Duration::from_secs(queue_poll_interval)).await;
    }
}
