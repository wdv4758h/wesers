extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate clap;


use clap::App as ClapApp;              // CLI arguments
use actix_web::{server, App, Path};


fn index(info: Path<(String, u32)>) -> String {
   format!("Hello {}! id:{}", info.0, info.1)
}

fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("cli.yml");
    let arguments = ClapApp::from_yaml(yml).get_matches();

    let address = format!("{}:{}", arguments.value_of("ip").unwrap(),
                                   arguments.value_of("port").unwrap());


    ////////////////////
    // Start Server
    ////////////////////

    env_logger::init();

    server::new(
        || App::new()
            .resource("/{name}/{id}/index.html", |r| r.with(index)))
        .bind(address.as_str()).unwrap()
        .run();
}
