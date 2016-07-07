#[macro_use]
extern crate clap;

use std::path::Path;

use clap::{App, Shell};


fn main() {
    let cargo_target_dir = Path::new(env!("OUT_DIR"))
                                .parent().unwrap()
                                .parent().unwrap()
                                .parent().unwrap()
                                .as_os_str();
    let yml = load_yaml!("src/arguments.yml");    // FIXME, make a function
    let mut app = App::from_yaml(yml);
    app.gen_completions("wesers",           // bin name
                        Shell::Bash,        // target shell
                        cargo_target_dir);  // writing path
}
