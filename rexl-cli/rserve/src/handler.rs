use crate::assets::{format_html, format_li};
use actix_web::body::Body;
use actix_web::web::Bytes;
use actix_web::{HttpRequest, HttpResponse, Responder};
use rexl::io::{abs_path, base_name, ext_name};
use rexl::mime::mime_from_filename;
use std::fs::{metadata, read_dir, File};
use std::io::Read;

pub async fn index(req: HttpRequest) -> impl Responder {
    match index_at(req.path()) {
        Err(v) => index_error(v),
        Ok(v) => v,
    }
}

pub fn index_at(url_path: &str) -> Result<HttpResponse, String> {
    let current_dir = abs_path(".")?;
    let file_path = format!("{}{}", &current_dir, url_path);
    println!("access {} via {}", file_path, url_path);

    let meta = metadata(&file_path).map_err(|err| err.to_string())?;

    if meta.is_file() {
        index_file(&file_path, meta.len() as usize)
    } else if meta.is_dir() {
        index_dir(&file_path, url_path)
    } else {
        Err(format!("unsupported file type {}", &file_path))
    }
}

fn index_file(p: &str, file_len: usize) -> Result<HttpResponse, String> {
    let mut file = File::open(p).map_err(|err| err.to_string())?;
    let mut buf = Vec::with_capacity(file_len);
    file.read_to_end(&mut buf).map_err(|err| err.to_string())?;

    let mut resp = HttpResponse::Ok();
    if let Some(mime) = mime_from_filename(p) {
        resp.header("Content-Type", mime);
    }
    Ok(resp.body(Body::Bytes(Bytes::from(buf))))
}

fn index_dir(file_path: &str, url_path: &str) -> Result<HttpResponse, String> {
    let mut lis = vec![];
    let rd = read_dir(file_path).map_err(|err| err.to_string())?;
    for r in rd {
        let entry = r.map_err(|err| err.to_string())?;
        let filename = entry.file_name().into_string().map_err(|err| {
            format!(
                "error occurred on {}/{}!",
                file_path,
                err.to_string_lossy().to_string()
            )
        })?;

        let ft = entry.file_type().map_err(|err| err.to_string())?;
        let clazz = if ft.is_file() {
            format!("file {}", ext_name(&filename))
        } else {
            "folder".to_owned()
        };

        let path = if url_path.ends_with("/") {
            format!("{}{}", url_path, &filename)
        } else {
            format!("{}/{}", url_path, &filename)
        };
        lis.push(format_li(&filename, &path, &clazz));
    }

    let title = base_name(file_path);
    let html = format_html(title, url_path, &lis.join("\n"));
    Ok(HttpResponse::Ok().body(html))
}

fn index_error(err: String) -> HttpResponse {
    HttpResponse::NotFound().body(format!("<h1>{}</h1>", err))
}
