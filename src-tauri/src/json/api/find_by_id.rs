use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde_json::Value;
use tiny_http::Response;

use super::util::create_json_response;

pub fn find_by_id_json( dataset: &Arc<Mutex<Vec<Value>>>,
    path_param: &HashMap<String, String>) -> Response<std::io::Cursor<Vec<u8>>> {
                let id =  path_param.get("id")
                    .and_then(|id| id.parse::<String>().ok());
                let data = dataset.lock().unwrap();
                if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"].to_string() == id)) {
                    let json = serde_json::to_string(item).unwrap();
                    create_json_response(&json, 200)
                } else { 
                    create_json_response(r#"{"error": "Item not found"}"#, 404)
                }
    }