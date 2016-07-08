#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate logger;

extern crate rustc_serialize;   // Needed by mustache template
extern crate mustache;          // Template


use std::str;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

use clap::App;              // CLI arguments
use iron::prelude::*;
use iron::{Handler, status};
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use staticfile::Static;     // middleware
use mount::Mount;           // middleware
use logger::Logger;         // middleware


#[derive(RustcEncodable)]
struct Link {
    url: String,
    name: String,
}


fn visit_dirs(dir: &Path) -> Vec<Link> {

    let mut data = vec![];

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

        let mut abs_url = "/".to_string();  // /xxx/ooo/ or /xxx/ooo
        let mut name = name.to_string();    // ooo/ or ooo
        abs_url.push_str(url.as_str());
        abs_url.push_str(trailing);
        name.push_str(trailing);

        data.push(
            Link {
                url: abs_url,
                name: name,
            }
        )
    }

    data
}


struct Wesers {
    detect_index: bool,
    template_dir: Option<String>,
}

impl Wesers {
    pub fn new(detect_index: bool,
               template_dir: Option<String>) -> Wesers {
        Wesers {
            detect_index: detect_index,
            template_dir: template_dir,
        }
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

            let dir_data = visit_dirs(path);

            // TODO, make it static
            let template;
            if let Some(ref template_dir) = self.template_dir {
                // custom template
                template = mustache::compile_path(Path::new(template_dir))
                                    .unwrap();
            } else {
                // default template
                let default = include_str!("default.mustache");
                template = mustache::compile_str(default);
            }

            let mut data = HashMap::new();
            data.insert("links", dir_data);

            let mut bytes = vec![];
            template.render(&mut bytes, &data).unwrap();
            let result = str::from_utf8(&bytes).unwrap();

            response = Response::with((status::Ok, result));
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
                                     .unwrap(),
                            arguments.value_of("template")
                                     .map(|s| s.to_string())
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
