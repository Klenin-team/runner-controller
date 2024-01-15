# Controller for running tests in RATS

This is repository with controller, that receives solutions from RATS backend and tests it.

This project is using [sunwalker-box](https://github.com/purplesyringa/sunwalker-box) as sandbox.
Actually, I'm not sure how sunwalker-box working inside, but currently this controller has to be run as root.

## 📝 Requirements
- Rust
- [sunwalker-box](https://github.com/purplesyringa/sunwalker-box) (added to `$PATH`)

## 🛠️ Building
1. Clone this repository
2. `$ cargo build --release`
3. Find compiled file at target/release/runner-controller

## ⚙️ Configuring
Example of configuration can be found in [config.yaml](config.yaml)
### 🖥 Language list
Unfortunately, language list is hardcoded, but you can easily change it and recompile:
1. Go to [src/structs/languages.rs](src/structs/languages.rs)
2. Add or remove languages
#### Fields:
```
languages.insert("{key}", Language {
        filename: "{filename}",
        compilible: {compilible},
        compile_command: vec![{compile comand}],
        execute_command: vec![{execute command}]
    });
```
- `{key}` -- language id (the same as at [backend](https://github.com/Klenin-team/RATS-web))
- `{filename}` -- where will be source code file located, eg. `solution.py`
- `{compilible}` -- `true`, if this language is compiled language. Else `false`
- `{compile command}` -- array with command to compile source code and put it into `/space/a.out`. Each arguments should be string and the first is the full path to compiler on your server. Eg. `"/usr/bin/rustc", "-o", "/space/a.out", "/space/solution.rs"`. If this is not compiled language -- leave empty
- `{execute command}` -- array with command to execute source code, like `{compile command}`. Eg. `"/usr/bin/python", "/space/solution.py"`. If the language is compiled, put `"/space/a.out"`

### 👥 [Queue service](https://github.com/Klenin-team/queue-service) location
Queue service location is configured through [config.yaml](config.yaml)
- `queue_url` -- url of your queue service instance **(required)**
- `verdicts_return_url` -- url, where verdicts should be send **(required)** 
- `queue_poll_interval` -- how many seconds to wait between requests to queue (default is 10)
### 💽 Available cores
Which CPU cores should be used for testing. This cores will be isolated
- `cores` -- list of numbers of CPU cores (indexing starts from 0) **(required)**
### 🗂 Sunbox location and chroot
- `sunwalker_path` -- path to your compiled sunwalker file (default value is `sunwalker_box`, that means, that sunwalker_box added to path)
- `root` -- path to directory, that will be root for sandbox (probably chroot with installed languages from [language list](#language-list)) (default `/`)


## 🏃 Running
To run this app you must to have [sunwalker-box](https://github.com/purplesyringa/sunwalker-box) built and added to your path. Also, `config.yaml` must be located in your working directory and because this program starts sunwalker-box, it **have to be started as root too**.
```bash
./runner-controller
```
## 🛑 Stopping
This code does not implement freeing cores after stopping, so you have to run
```bash
sunwalker_box free --core {core}
```
for every core in `config.yaml`. If you're restarting this program, you don't need to do this.


## 📋 To do list
- [x] compiling
- [x] running compiled code
- [x] catching exceeding limits
- [x] checking output
- [x] configuration files (partially, supported languages are hardcoded (but can be easely edited at src/structs/languages.rs))
- [x] connection to backend with tasks
- [ ] ~~comments~~ (nevermind)
