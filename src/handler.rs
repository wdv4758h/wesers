use std::str;
use std::fs;
use std::path::{Path, PathBuf};
use std::env;

use iron::prelude::*;
use iron::{Handler, status};
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use staticfile::Static;     // middleware
use mount::Mount;           // middleware
use mustache::{self, MapBuilder, VecBuilder};


#[derive(RustcEncodable)]
struct Link {
    url: String,
    name: String,
}


fn visit_dirs(dir: &Path) -> VecBuilder {

    let mut data = VecBuilder::new();

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

        data = data.push(
            &Link {
                url: abs_url,
                name: name,
            }
        ).ok().unwrap();
    }

    data
}


pub struct Wesers {
    detect_index: bool,
    template: mustache::Template,
    mount: Mount,
}

impl Wesers {
    pub fn new(detect_index: bool,
               template_path: Option<String>,
               root_dir: &str) -> Wesers {

        let template = {
            if let Some(ref template_path) = template_path {
                // custom template
                mustache::compile_path(Path::new(template_path))
                         .unwrap()
            } else {
                // default template
                let default = include_str!("default.mustache");
                mustache::compile_str(default)
            }
        };

        env::set_current_dir(Path::new(root_dir)).unwrap();

        let mount = {
            let mut m = Mount::new();
            m.mount("/", Static::new(Path::new(".")));
            m
        };

        Wesers {
            detect_index: detect_index,
            template: template,
            mount: mount,
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

            let mut data = MapBuilder::new()
                    .insert_vec("links", |_| visit_dirs(path))
                    .insert_str("current_dir",
                                format!("/{}", path.to_str().unwrap()))
                    .build();

            let mut bytes = vec![];
            self.template.render_data(&mut bytes, &data);
            let result = str::from_utf8(&bytes).unwrap();

            response = Response::with((status::Ok, result));
            response.headers.set(ContentType::html());

        } else {

            response = self.mount.handle(req).unwrap();

        }

        Ok(response)
    }
}
