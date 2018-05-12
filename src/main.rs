#[macro_use]
extern crate clap;      // CLI arguments
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate actix_web;
extern crate env_logger;
extern crate openssl;
extern crate mustache;


mod handler;


use actix_web::{server, fs, App};
use clap::App as ClapApp;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("cli.yml");
    let arguments = ClapApp::from_yaml(yml).get_matches();

    let address = format!("{}:{}", arguments.value_of("ip").unwrap(),
                                   arguments.value_of("port").unwrap());

    let static_root = arguments.value_of("root").unwrap().to_string();


    ////////////////////
    // Start Server
    ////////////////////

    env_logger::init();

    let wesers_server = server::new(
        move || App::new()
            .handler("/",
                     fs::StaticFiles::new(&static_root)
                        .show_files_listing()
                        .files_listing_renderer(handler::wesers_file_listing)));

    if arguments.occurrences_of("https") > 0 {
        #[cfg(feature = "https")]
        {
            use std::path::PathBuf;

            println!("Simple HTTPS Server running on https://{}/", address);
            let cert = PathBuf::from(arguments.value_of("cert").unwrap());
            let key = PathBuf::from(arguments.value_of("key").unwrap());

            let mut builder =
                SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(key, SslFiletype::PEM)
                .unwrap();
            builder
                .set_certificate_chain_file(cert)
                .unwrap();
            wesers_server.bind_ssl(address.as_str(), builder).unwrap().run();
        }
        #[cfg(not(feature = "https"))]
        {
            println!("To use HTTPS, you need to compile with 'https' feature");
        }
    } else {
        println!("Simple HTTP Server running on http://{}/", address);
        wesers_server.bind(address.as_str()).unwrap().run();
    }
}
