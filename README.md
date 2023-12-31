# Controller for running tests in RATS

This is repository with controller, that receives solutions from RATS backend and tests it.

This project is using [sunwalker-box](https://github.com/purplesyringa/sunwalker-box) as sandbox.
Actually, I'm not sure how sunwalker-box working inside, but currently this controller has to be run as root.

## Requirements
- Rust
- [sunwalker-box](https://github.com/purplesyringa/sunwalker-box) (added to `$PATH`)

## To do list
- [ ] compiling
- [x] running compiled code
- [x] catching exceeding limits
- [x] checking output
- [ ] configuration files
- [ ] connection to backend with tasks
- [ ] **comments**
