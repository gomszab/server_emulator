use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde_json::Value;
use tiny_http::Response;

use super::util::create_json_response;

pub fn remove_by_id_json( dataset: &Arc<Mutex<Vec<Value>>>,
    path_param: &HashMap<String, String>) -> Response<std::io::Cursor<Vec<u8>>> {
            let id = path_param.get("id");

        let mut data = dataset.lock().unwrap();
        if let Some(pos) = id.and_then(|id| data.iter().position(|item| item["id"].to_string() == *id)) {
            data.remove(pos);
            create_json_response(r#"{"error": "Item removed"}"#, 200)
        } else {
            create_json_response(r#"{"error": "Item not found"}"#, 404)
        }
    }