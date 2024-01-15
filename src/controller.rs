use std::collections::{ HashMap, LinkedList };

use json::object;
use tokio::sync::{ mpsc, oneshot };
use crate::structs;

pub fn json_to_solution(text: &str, languages: &HashMap<&str, structs::Language>) 
        -> Result<(u128, structs::Solve), ()> {
    let json = json::parse(text).expect("");
    if json["any"] == false {
        return Err(())
    }
 
    let id = json["id"].to_string();
    let id = id.parse::<u128>();
    if id.is_err() {
        return Err(()) 
    }
    let id = id.unwrap();
 
    let code = json["code"].to_string();
 
    let language = json["language"].to_string();
    let language_as_struct = languages.get(language.as_str());
    if language_as_struct.is_none() {
        return Err(())
    }
    let language_as_struct = language_as_struct.unwrap();
    
    let use_stdio = json["stdio"].as_bool().unwrap_or(true);
    let input_file = json["input_file"].to_string();
    let output_file = json["output_file"].to_string();
 
    
    let mut tests_list: LinkedList<structs::Test> = LinkedList::new();
    for json_test in json["tests"].members() {
        tests_list.push_back(structs::Test { input: json_test[0].to_string(), output: json_test[1].to_string() })
    }
    
    let memory_limit = json["memory_limit"].as_u64().unwrap_or(134217728);
    let time_limit = json["time_limit"].as_f32().unwrap_or(10.0);
 
    let solution = structs::Solve{
        code,
        stdio: use_stdio,
        input_name: input_file,
        output_name: output_file,
        tests: tests_list,
        language: language_as_struct.clone(),
        time_limit,
        memory_limit
    };
    Ok((id, solution))

}

pub async fn run(
        sender: &mpsc::Sender<(structs::Solve, oneshot::Sender<Vec<structs::Verdict>>)>, 
        solution: structs::Solve, freed_core_tx: mpsc::Sender<u8>, using_core: u8,
        queue_base_url: String, id: u128) {
    let (resp_tx, resp_rx) = oneshot::channel();
    let res = sender.send((solution, resp_tx)).await;
    if res.is_err() {
        tokio::spawn(async move {
            send_back_results(vec![structs::Verdict{ used_time: 0.0, used_memory: 0, verdict: structs::Verdicts::SE }],
                              Some(freed_core_tx), Some(using_core), 
                              queue_base_url, id).await;
        });
        return ();
    }
    tokio::spawn(async move {
        let verdicts = resp_rx.await;

        if verdicts.is_err() {
            send_back_results(vec![structs::Verdict{ used_time: 0.0, used_memory: 0, verdict: structs::Verdicts::SE }],
                              Some(freed_core_tx), Some(using_core),
                              queue_base_url, id).await;
            return ();
        }
        send_back_results(verdicts.unwrap(), Some(freed_core_tx), Some(using_core),
                          queue_base_url, id).await
    });
}

pub async fn send_back_results(verdicts: Vec<structs::Verdict>,
                               freed_core_tx: Option<mpsc::Sender<u8>>, using_core: Option<u8>,
                               queue_base_url: String, id: u128) {
    if freed_core_tx.is_some() && using_core.is_some() {
        freed_core_tx.unwrap().send(using_core.unwrap()).await.expect("This core not gonna be used((");
    }

    let mut string_verdicts = Vec::new();
    for verdict in verdicts {
        string_verdicts.push(object! {
            used_memory: verdict.used_memory,
            used_time: verdict.used_time,
            verdict: match verdict.verdict {
                structs::Verdicts::OK => "OK",
                structs::Verdicts::RE => "RE",
                structs::Verdicts::TL => "TL",
                structs::Verdicts::ML => "ML",
                structs::Verdicts::WA => "WA",
                structs::Verdicts::CE => "CE",
                structs::Verdicts::SE => "SE"
            }
        });
    }

    let form = object! {
        id: id.to_string(),
        verdicts: string_verdicts
    };

    let client = reqwest::Client::new();
    client.post(queue_base_url.to_string() + "/solution/checked")
        .body(form.dump())
        .header("Content-Type", "application/json")
        .send()
        .await.expect("Seems like queue unreachable");
}

