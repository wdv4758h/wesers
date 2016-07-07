#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate logger;

use std::fs;
use std::path::Path;

use clap::App;              // CLI arguments
use iron::prelude::*;
use iron::{Handler, status};
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use staticfile::Static;     // middleware
use mount::Mount;           // middleware
use logger::Logger;         // middleware


fn visit_dirs(dir: &Path) -> Option<String> {

    let mut html = "".to_string();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let url = entry.path()
                       .to_str()
                       .unwrap()
                       .to_string();    // FIXME
        let name = url.rsplitn(2, '/').collect::<Vec<_>>()[0];
        let trailing: &str;

        if fs::metadata(entry.path()).unwrap().is_dir() {
            trailing = "/";
        } else {
            trailing = "";
        }

        html.push_str(format!("<li><a href='/{url}{tr}'>{name}{tr}</a></li>",
                              url = url,
                              tr = trailing,
                              name = name).as_str());
    }

    Some(html)
}


struct Wesers {
    detect_index: bool,
}

impl Wesers {
    pub fn new(detect_index: bool) -> Wesers {
        Wesers { detect_index: detect_index }
    }
}

impl Handler for Wesers {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut path = req.url.path.join("/");

        if path.is_empty() {
            path = ".".to_string();
        }

        let path = Path::new(path.as_str());

        let metadata = fs::metadata(&path);
        if metadata.is_err() {
            // 404
            return Ok(Response::with(status::NotFound));
        }

        let mut is_dir = metadata.unwrap().is_dir();

        if is_dir {

            ////////////////////
            // add trailing slash
            // /xxx -> /xxx/
            // ["target", "doc"] -> ["target", "doc", ""]
            ////////////////////

            if !req.url.path
                       .last()
                       .unwrap()
                       .is_empty() {
                req.url.path.push("".to_string());
                return
                    Ok(Response::with((status::MovedPermanently,
                                       Redirect(req.url.clone()))));
            }

            ////////////////////
            // detect index.html
            ////////////////////

            if self.detect_index {
                let index_path = format!("{}/index.html",
                                         &path.to_str()
                                              .unwrap()
                                              .trim_right_matches('/'));
                let index = Path::new(index_path.as_str());

                if fs::metadata(&index).is_ok() {
                    is_dir = false;
                    req.url.path.pop();
                    req.url.path.push("index.html".to_string());
                }
            }
        }


        let mut response;

        if is_dir {

            response = Response::with(
                                    (status::Ok,
                                     visit_dirs(path)
                                        .unwrap()
                                    )
                               );
            response.headers.set(ContentType::html());

        } else {

            lazy_static! {
                static ref MOUNT: Mount = {
                    let mut m = Mount::new();
                    m.mount("/", Static::new(Path::new(".")));
                    m
                };
            }

            response = MOUNT.handle(req).unwrap();

        }

        Ok(response)
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
                                     .unwrap()
                        )
                    );

    ////////////////////
    // Other Middlewares
    ////////////////////

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
