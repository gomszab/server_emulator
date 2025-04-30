use std::sync::Mutex;

use serde_json::Value;

pub struct DatasetFacade {
    pub(crate) data: Mutex<Vec<Value>>,
}

impl DatasetFacade {
    pub fn new(data: Vec<Value>) -> Self {
        Self {
            data: Mutex::new(data),
        }
    }
}
