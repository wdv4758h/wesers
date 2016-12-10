#[macro_use]
extern crate clap;

use std::env;
use std::path::Path;

use clap::{App, Shell};


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_target_dir = Path::new(&out_dir)
                                .parent().unwrap()
                                .parent().unwrap()
                                .parent().unwrap()
                                .as_os_str();
    let yml = load_yaml!("src/cli.yml");    // FIXME, make a function
    let mut app = App::from_yaml(yml);
    app.gen_completions("wesers",           // bin name
                        Shell::Bash,        // target shell
                        cargo_target_dir);  // writing path
}
