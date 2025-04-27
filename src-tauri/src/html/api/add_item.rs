use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use tiny_http::{Header, Response};

use crate::html::template_engine;

use super::util::{make_invalid_response_html, HTML_RESPONSE};



pub fn add_item_html(
    dataset: &Arc<Mutex<Vec<Value>>>,
    template: &String,
    request: &Option<Value>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut request_body = request.clone();
    {
        let mut data = dataset.lock().unwrap();
        
        if let Some(ref mut value) = request_body {
            if let Value::Object(ref mut map) = value {
                // Convert data.len() to JSON-compatible number
                map.insert("id".to_string(), json!((data.len() + 1) as u64));
                data.push(value.clone());
            } else {
                return make_invalid_response_html("<h1>Invalid request body</h1>");
            }
        } else {
            return make_invalid_response_html("<h1>Invalid request body</h1>");
        }
    }

    let content =
        template_engine::render_template(template.as_str(), Some(dataset), None, None, request_body);
    Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
}