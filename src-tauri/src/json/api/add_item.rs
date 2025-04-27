use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use tiny_http::Response;

use super::util::create_json_response;

pub fn add_item_json(
    dataset: &Arc<Mutex<Vec<Value>>>,
    request: &Option<Value>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let request_body = request.clone();
    let mut data = dataset.lock().unwrap();

    if let Some(mut json_value) = request_body {
        if let Value::Object(ref mut map) = json_value {
            map.insert("id".to_string(), json!((data.len() + 1) as u64));
            data.push(json_value.take());
            create_json_response(r#"{"message": "Item added"}"#, 201)
        } else {
            create_json_response(r#"{"message": "Invalid json"}"#, 400)
        }
    } else {
        create_json_response(r#"{"message": "Invalid json"}"#, 400)
    }
}
