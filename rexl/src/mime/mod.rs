use crate::text::load_properties_from_str;
use std::collections::HashMap;

pub const MEDIA_TYPES: &'static str = include_str!("./media-types.properties");

pub fn mime_map() -> HashMap<String, String> {
    load_properties_from_str(MEDIA_TYPES)
}

use std::cell::RefCell;

thread_local!(static MIME_MAP: RefCell<HashMap<String, String>> = RefCell::new(mime_map()));

pub fn mime_from_filename(filename: &str) -> Option<String> {
    let ext = match filename.rfind('.') {
        Some(i) => filename[(i + 1)..].to_string(),
        None => filename.to_string(),
    };

    MIME_MAP.with(|rc| rc.borrow().get(&ext).map(|v| v.clone()))
}
