use actix_web::body::Body;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, HttpResponse, Responder};
use rexl::mime::mime_from_filename;
use std::fs::{metadata, read_dir, File};
use std::io::Read;

pub async fn index(req: HttpRequest) -> impl Responder {
    let url_path = req.path();
    println!("access: {}", url_path);

    let file_path = format!(".{}", url_path);
    let meta = match metadata(&file_path) {
        Err(v) => {
            return index_error(v.to_string());
        }
        Ok(v) => v,
    };

    if meta.is_file() {
        index_file(&file_path, meta.len() as usize)
    } else if meta.is_dir() {
        index_dir(&file_path)
    } else {
        index_error(format!("unsupported file type {}", &file_path))
    }
}

fn index_file(p: &str, file_len: usize) -> HttpResponse {
    let mut file = match File::open(p).map_err(|err| err.to_string()) {
        Err(v) => return index_error(v),
        Ok(v) => v,
    };

    let mut buf = Vec::with_capacity(file_len);
    match file.read_to_end(&mut buf) {
        Err(v) => return index_error(v.to_string()),
        Ok(_) => {
            let mut resp = HttpResponse::Ok();
            if let Some(mime) = mime_from_filename(p) {
                resp.header("Content-Type", mime);
            }
            resp.body(Body::Bytes(Bytes::from(buf)))
        }
    }
}

fn index_dir(file_path: &str) -> HttpResponse {
    let mut lines = vec![format!("<h1>Index of {}</h1>", file_path,)];
    let rd = match read_dir(file_path).map_err(|err| err.to_string()) {
        Err(v) => return index_error(v),
        Ok(v) => v,
    };
    let mut c = 0;
    for r in rd {
        let entry = match r.map_err(|err| err.to_string()) {
            Err(v) => return index_error(v),
            Ok(v) => v,
        };
        let line = match entry
            .path()
            .to_str()
            .ok_or_else(|| format!("fail to read dir entry on dir {}", file_path))
        {
            Err(v) => return index_error(v),
            Ok(v) => v,
        }
        .to_string();

        let prefix = if c == 0 { "<ul>\n" } else { "" };

        let url_path = &line[1..];
        let filename = &line[(line.rfind('/').unwrap_or(0) + 1)..];
        lines.push(format!(
            "{}<li><a href='{}'>{}</a></li>",
            prefix, url_path, filename
        ));
        c += 1;
    }
    lines.push("</ul>\n".to_string());
    HttpResponse::Ok().body(lines.join("\n"))
}

fn index_error(err: String) -> HttpResponse {
    HttpResponse::NotFound().body(format!("<h1>{}</h1>", err))
}
