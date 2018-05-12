use mustache;
use std::path::Path;
use std::io::Error;
use std::fs as std_fs;
use actix_web::{fs, HttpRequest, HttpResponse, Responder};
use mustache::{MapBuilder, VecBuilder};


#[derive(Serialize)]
struct Link {
    url: String,
    name: String,
}

fn visit_dirs(dir: &Path) -> VecBuilder {

    let mut data = VecBuilder::new();
    let dir = if dir.to_str().unwrap().is_empty() { Path::new(".") } else { dir };

    for entry in std_fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let url = entry.path()
                       .to_str()
                       .unwrap()
                       .to_string();    // FIXME
        let name = url.rsplitn(2, '/').collect::<Vec<_>>()[0];

        let trailing = if std_fs::metadata(entry.path()).unwrap().is_dir() {
            "/"
        } else {
            ""
        };

        let mut abs_url = "/".to_string();  // /xxx/ooo/ or /xxx/ooo
        let mut name = name.to_string();    // ooo/ or ooo
        abs_url.push_str(url.trim_left_matches("./"));
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

pub fn wesers_file_listing(dir: &fs::Directory, req: &HttpRequest) -> Result<HttpResponse, Error> {
    // display index.html if dir has it
    let mut index_html = dir.path.clone();
    index_html.push("index.html");
    if std_fs::metadata(&index_html).is_ok() {
        return fs::NamedFile::open(index_html)?.respond_to(&req);
    }

    let related_path = dir.path.strip_prefix(&dir.base).unwrap();

    // otherwise display files listing with template engine render
    let data = MapBuilder::new()
        .insert_vec("links", |_| visit_dirs(&related_path))
        .insert_str("current_dir",
                    format!("/{}", related_path.to_str().unwrap()))
        .build();
    let template = {
        let template_path: Option<String> = None;
        if let Some(ref template_path) = template_path {
            // custom template
            mustache::compile_path(Path::new(template_path))
                     .unwrap()
        } else {
            // default template
            let default = include_str!("default.mustache");
            mustache::compile_str(default)
                     .unwrap()
        }
    };
    let mut bytes = vec![];
    template.render_data(&mut bytes, &data).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    Ok(result.respond_to(&req).unwrap())
}
