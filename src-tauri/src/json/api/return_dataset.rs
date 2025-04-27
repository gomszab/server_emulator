use std::sync::{Arc, Mutex};

use serde_json::Value;
use tiny_http::Response;

use super::util::create_json_response;

pub fn return_dataset_json(dataset: &Arc<Mutex<Vec<Value>>>) -> Response<std::io::Cursor<Vec<u8>>>{
    if let Ok(data) = dataset.lock(){
        if let Ok(json) = serde_json::to_string(&*data) {
            create_json_response(&json, 200)
        }else{
            create_json_response(r#"{"message": "Something wrong"}"#, 400)
        }
    }else{
        create_json_response(r#"{"message": "Something wrong"}"#, 503)
    }
}