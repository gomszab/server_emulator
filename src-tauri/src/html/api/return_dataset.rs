use std::sync::{Arc, Mutex};

use serde_json::Value;
use tiny_http::{Header, Response};

use crate::html::template_engine;

use super::util::HTML_RESPONSE;

pub fn return_dataset_html(
    dataset: &Arc<Mutex<Vec<Value>>>,
    template: &String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content =
        template_engine::render_template(template.as_str(), Some(dataset), None, None, None);
    Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
}
