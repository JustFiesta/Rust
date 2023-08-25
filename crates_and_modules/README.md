
# Excercise to learn how crates and modules work from https://practice.rs/crate-module/module.html

#### So at start we are given file lib.rs. We want to move modules to different files and directories. Final goal is to have dir tree like this: 

- Cargo.toml
- src
  - back_of_house.rs
  - front_of_house
    - hosting.rs
    - mod.rs
    - serving.rs
  - lib.rs
  - main.rs


## To do this:

1. make according files to tree
2. move modules from lib to back_of_house.rs and leave addnotation to them in lib.rs (pub/priv mod back_of_house;)
3. create directories and make files, than move according modules to this files.
4. create mod.rs and place there modules in this dir (pub/priv mod module_name) - it makes front_of_house directory visable to main as module, so we can use it's modules.
5. check in main if we can access modules.

* [1] file to move modules from -> move_from_lib.rs

### It was fun excercise and helps to understand how crates and modules work in rust
