use std::fs::canonicalize;
use std::path::Path;

pub fn base_name(p: &str) -> &str {
    let path = if p.ends_with("/") {
        &p[0..p.len() - 1]
    } else {
        p
    };

    if path.len() == 0 {
        return "/"
    }

    match path.rfind('/') {
        None => path,
        Some(i) => &path[(i + 1)..],
    }
}

#[inline]
pub fn ext_name(p: &str) -> &str {
    match p.rfind('.') {
        None => p,
        Some(i) => &p[(i + 1)..],
    }
}

#[inline]
pub fn abs_path<P: AsRef<Path>>(path: P) -> Result<String, String> {
    canonicalize(path)
        .map_err(|err| err.to_string())?
        .into_os_string()
        .into_string()
        .map_err(|err| err.to_string_lossy().to_string())
}
