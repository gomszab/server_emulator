use std::sync::Mutex;

use serde_json::{json, Value};

use super::{
    datasetstruct::DatasetFacade,
    types::{ElementNotFound, InvalidRequest},
};

pub trait DatasetWriteOperation {
    fn add_item(&self, request: Value) -> Result<&Mutex<Vec<Value>>, InvalidRequest>;

    fn remove_by_id(&self, id: String) -> Result<&Mutex<Vec<Value>>, ElementNotFound>;
}

impl DatasetWriteOperation for DatasetFacade {
    fn add_item(&self, request: Value) -> Result<&std::sync::Mutex<Vec<Value>>, InvalidRequest> {
        let mut request_body = request.clone();

        let success = if let Value::Object(ref mut map) = request_body {
            let mut data = self.data.lock().unwrap();
            // Convert data.len() to JSON-compatible number
            map.insert("id".to_string(), json!((data.len() + 1) as u64));
            data.push(Value::Object(map.clone()));
            true
        } else {
            false
        };
        if success {
            Ok(&self.data)
        } else {
            Err(InvalidRequest)
        }
    }

    fn remove_by_id(&self, id: String) -> Result<&Mutex<Vec<Value>>, ElementNotFound> {
        let success = {
            let mut data = self.data.lock().unwrap();
            if let Some(pos) = data.iter().position(|item| item["id"].to_string() == id) {
                data.remove(pos);
                true
            } else {
                false
            }
        };
        if success {
            Ok(&self.data)
        } else {
            Err(ElementNotFound)
        }
    }
}
