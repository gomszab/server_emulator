use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use tiny_http::{Header, Response};

use crate::html::template_engine;

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn find_by_id_html(
    dataset: &Arc<Mutex<Vec<Value>>>,
    template: &String,
    path_param: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let id = path_param.get("id");
    let data = dataset.lock().unwrap();

    if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"].to_string() == *id)) {
        let content = template_engine::render_template(
            template.as_str(),
            Some(item),
            Some(path_param.clone()),
            None,
            None,
        );
        Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
    } else {
        return make_invalid_response_html("<h1>Invalid id</h1>");
    }
}
