use std::{collections::HashMap, sync::Mutex};

use serde_json::Value;

use crate::util::get_value_as_string;

use super::{datasetstruct::DatasetFacade, types::ElementNotFound};

pub trait DatasetReadOperation {
    fn get_by_id(&self, path_param: &String) -> Result<Value, ElementNotFound>;

    fn get_by_parameters(
        &self,
        parameters: &HashMap<String, String>,
    ) -> Result<Value, ElementNotFound>;

    fn get_all(&self) -> Result<&Mutex<Vec<Value>>, ()>;
}

impl DatasetReadOperation for DatasetFacade {
    fn get_by_id(&self, id: &String) -> Result<Value, ElementNotFound> {
        let data = self.data.lock().unwrap();

        match data.iter().find(|item| item["id"].to_string() == *id) {
            Some(item) => Ok(item.clone()),
            _ => Err(ElementNotFound),
        }
    }

    fn get_by_parameters(
        &self,
        parameters: &HashMap<String, String>,
    ) -> Result<Value, ElementNotFound> {
        let data = self.data.lock().unwrap();

        match data.iter().find(|item| {
            let mut counter = 0;
            let all = &parameters.len();
            for query_param in parameters {
                if get_value_as_string(&item[query_param.0]) == query_param.1.to_string() {
                    counter += 1;
                }
            }
            return counter == *all;
        }) {
            Some(item) => Ok(item.clone()),
            _ => Err(ElementNotFound),
        }
    }

    fn get_all(&self) -> Result<&Mutex<Vec<Value>>, ()> {
        return Ok(&self.data);
    }
}
