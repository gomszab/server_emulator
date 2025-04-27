use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use tiny_http::Response;

use crate::util::get_value_as_string;

use super::util::create_json_response;

pub fn find_by_queryparam_json(
    dataset: &Arc<Mutex<Vec<Value>>>,
    query_parameter: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let data = dataset.lock().unwrap();

    if let Some(item) = data.iter().find(|item| {
        let mut counter = 0;
        let all = &query_parameter.len();
        for query_param in query_parameter {
            if get_value_as_string(&item[query_param.0]) == query_param.1.to_string() {
                counter += 1;
            }
        }
        return counter == *all;
    }) {
        let json = serde_json::to_string(item).unwrap();
        create_json_response(&json, 200)
    } else {
        create_json_response(r#"{"error": "Item not found"}"#, 404)
    }
}
