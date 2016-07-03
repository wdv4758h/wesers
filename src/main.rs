#[macro_use]
extern crate clap;

extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate logger;

use std::path::Path;

use clap::App;              // CLI arguments
use iron::prelude::*;
use staticfile::Static;     // middleware
use mount::Mount;           // middleware
use logger::Logger;         // middleware


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("src/arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();

    let address = format!("{}:{}", arguments.value_of("ip").unwrap(),
                                   arguments.value_of("port").unwrap());

    ////////////////////
    // Handler
    ////////////////////

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(".")));

    ////////////////////
    // Other Middlewares
    ////////////////////

    let mut chain = Chain::new(mount);

    let (logger_before, logger_after) = Logger::new(None);

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);

    // Link logger_after as your *last* after middleware.
    chain.link_after(logger_after);

    ////////////////////
    // Start Server
    ////////////////////

    println!("Simple HTTP Server running on http://{}/", address);
    Iron::new(chain).http(address.as_str()).unwrap();
}
