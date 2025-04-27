use std::collections::HashMap;

use serde_json::Value;
use tiny_http::{Header, Response};

use crate::html::template_engine;

use super::util::HTML_RESPONSE;

pub fn no_logic_html(
    template: &String,
    query_parameter: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = template_engine::render_template::<&Value>(
        template.as_str(),
        None,
        None,
        Some(query_parameter.clone()),
        None,
    );
    Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
}