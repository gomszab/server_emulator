use std::sync::Mutex;

use serde_json::Value;

use crate::util::get_value_as_string;

pub struct DatasetFacade {
    pub(crate) data: Mutex<Vec<Value>>,
    pub(crate) id_prefix: String,
}

impl DatasetFacade {
    pub fn new(data: Vec<Value>, id_prefix: String) -> Self {
        Self {
            data: Mutex::new(append_prefix(data, &id_prefix)),
            id_prefix,
        }
    }
}

fn append_prefix(mut data: Vec<Value>, prefix: &String) -> Vec<Value> {
    for item in data.iter_mut() {
        if let Some(obj) = item.as_object_mut() {
            if let Some(id_val) = obj.get_mut("id") {
                *id_val = Value::String(format!(
                    "{}_{}",
                    prefix,
                    get_value_as_string(id_val).as_str()
                ));
            }
        }
    }
    data
}
