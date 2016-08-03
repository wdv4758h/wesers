#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate clap;              // CLI arguments
extern crate iron;              // web framework
extern crate logger;            // Iron's middleware
extern crate staticfile;        // Iron's middleware
extern crate mount;             // Iron's middleware
extern crate rustc_serialize;   // needed by mustache template
extern crate mustache;          // mustache template
extern crate term;

mod handler;

use clap::App;              // CLI arguments
use iron::prelude::*;       // web framework
use logger::Logger;         // middleware
use logger::format::{Format, FormatColor};
use term::color;

use handler::Wesers;        // my handler


// from iron logger
fn status_color(_req: &Request, res: &Response) -> Option<color::Color> {
    use iron::status::StatusClass::*;
    use iron::status::NotFound;

    match res.status.unwrap_or(NotFound).class() {
        Informational   => Some(color::BLUE),
        Success         => Some(color::GREEN),
        Redirection     => Some(color::YELLOW),
        ClientError     => Some(color::RED),
        ServerError     => Some(color::BRIGHT_RED),
        NoClass         => None
    }
}

fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("arguments.yml");
    let arguments = App::from_yaml(yml).get_matches();

    let address = format!("{}:{}", arguments.value_of("ip").unwrap(),
                                   arguments.value_of("port").unwrap());

    ////////////////////
    // Handler
    ////////////////////

    let mut chain = Chain::new(
                        Wesers::new(
                            arguments.value_of("index")
                                     .unwrap()
                                     .parse()
                                     .unwrap(),
                            arguments.value_of("template")
                                     .map(|s| s.to_string()),
                            arguments.value_of("root")
                                     .unwrap()
                        )
                    );

    ////////////////////
    // Other Middlewares
    ////////////////////

    let format =
        Format::new(
            "{ip-addr} @[bold]{method}@ {uri} @[bold]->@ @[C]{status}@ ({response-time})",
            vec![FormatColor::FunctionColor(status_color)], vec![]).unwrap();
    let (logger_before, logger_after) = Logger::new(Some(format));

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);

    // Link logger_after as your *last* after middleware.
    chain.link_after(logger_after);

    ////////////////////
    // Start Server
    ////////////////////

    if arguments.occurrences_of("https") > 0 {
        #[cfg(feature = "https")]
        {
            use std::path::PathBuf;

            println!("Simple HTTPS Server running on https://{}/", address);
            let cert = PathBuf::from(arguments.value_of("cert").unwrap());
            let key = PathBuf::from(arguments.value_of("key").unwrap());
            Iron::new(chain).https(address.as_str(), cert, key).unwrap();
        }
        #[cfg(not(feature = "https"))]
        {
            println!("To use HTTPS, you need to compile with 'https' feature");
        }
    } else {
        println!("Simple HTTP Server running on http://{}/", address);
        Iron::new(chain).http(address.as_str()).unwrap();
    }
}
