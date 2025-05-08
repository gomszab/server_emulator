use std::sync::Mutex;

use rand::random_range;
use serde_json::Value;

use crate::util::get_value_as_string;

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

        let rand_id: u64 = random_range(100..3000);

        let id = format!("{}_{}", &self.id_prefix, rand_id);

        let success = if let Value::Object(ref mut map) = request_body {
            let mut data = self.data.lock().unwrap();
            // Convert data.len() to JSON-compatible number
            map.insert("id".to_string(), Value::String(id));
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
            if let Some(pos) = data
                .iter()
                .position(|item| get_value_as_string(&item["id"]) == id)
            {
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
