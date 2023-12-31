use json::{object, JsonValue};
use tokio::process::Command;
use tokio::io::AsyncWriteExt;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::{ChildStdin, ChildStdout};
use std::process::Stdio;
use tokio::io::Lines;
use std::str;

/*
 * This is the type, for interracting directly with sandbox
 */

pub struct Api {
    stdin: ChildStdin,
    reader: Lines<BufReader<ChildStdout>>
}

impl Api {
    pub async fn new(core: u8) -> Self {

        Command::new("sunwalker_box")
            .args(&["isolate", "--core", &core.to_string()])
            .spawn()
            .expect("failed to start")
            .wait()
            .await.expect("whoops");

        let mut cmd = Command::new("sunwalker_box");
        cmd.args(&["start", "--core", &core.to_string()]);

        cmd.stdout(Stdio::piped());
        cmd.stdin(Stdio::piped());

        let mut child = cmd.spawn().expect("failed to start");

        let stdin = child.stdin.take().expect("no stdin");
        let stdout = child.stdout.take().expect("no stdout");
        let reader = BufReader::new(stdout).lines();
        Self {
            stdin,
            reader
        }
    }

    pub async fn create_file(&mut self, filename: &str, file: &str) {
        let data = object! {
            path: format!("/space/{}", filename),
            content: file.as_bytes()
        };
        self.submit_and_await("mkfile", data).await;
    }

    pub async fn read_file(&mut self, filename: &str) -> String {
        let data = object! {
            path: format!("/space/{}", filename),
            at: 0,
            len: 0
        };
        let answer = json::parse(&self.submit_and_await("cat", data).await[3..]).expect("not valid json");
        let mut answer_arr = Vec::new();
        for i in answer.members() {
            answer_arr.push(i.as_u8().expect("not u8"));
        }
        let out = str::from_utf8(answer_arr.as_slice().clone()).expect("whoops").to_owned();
        out
    }

    pub async fn run(&mut self, command: Vec<&'static str>, redirect_stdio: Option<bool>,
                     cpu_limit: Option<f32>, memory_limit: Option<u64>) -> JsonValue {
        let redirect_stdio = redirect_stdio.unwrap_or(false);
        /* Default limits to prevent DOS attacks */
        let cpu_limit = cpu_limit.unwrap_or(10.0);
        let memory_limit = memory_limit.unwrap_or(128000000);
        let mut data = object! {
            argv: command,
            real_time_limit: cpu_limit,
            memory_limit: memory_limit
        };
        if redirect_stdio {
            data["stdin"] = "/space/input.txt".into();
            data["stdout"] = "/space/output.txt".into();
            data["stderr"] = "/space/errput.txt".into();
        }
        let out = self.submit_and_await("run", data).await;
        json::parse(&out[3..]).unwrap()
    }


    async fn submit_and_await(&mut self, command: &str, data: JsonValue) -> String {
        self.stdin.write(command.as_bytes()).await.expect("write error");
        self.stdin.write(b" ").await.expect("write error");
        self.stdin.write(data.dump().as_bytes()).await.expect("write error");
        self.stdin.write(b"\n").await.expect("write error");
        self.reader.next_line().await.expect("nope").unwrap()
    }
}