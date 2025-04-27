use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;

use crate::util::RequestWrapper;

pub type JsonResponse = (Arc<Mutex<Vec<Value>>>, JsonTemplate);
pub enum JsonTemplate {
    ReturnDataSet(),
    AddItem(Option<Value>),
    FindById(HashMap<String, String>),
    FindByQueryParameter(HashMap<String, String>),
    RemoveById(HashMap<String, String>),
    Unsupported(),
}

impl JsonTemplate {
    pub fn create(wrapper: RequestWrapper) -> JsonTemplate {
        if let Some(logic) = &wrapper.endpoint.logic {
            match logic.as_str() {
                "add item" => JsonTemplate::AddItem(wrapper.request),
                "return dataset" => JsonTemplate::ReturnDataSet(),
                "find by id in dataset" => JsonTemplate::FindById(wrapper.path_params),
                "find in dataset by queryparameter" => {
                    JsonTemplate::FindByQueryParameter(wrapper.query_params)
                }
                "remove by id in dataset" => JsonTemplate::RemoveById(wrapper.path_params),
                _ => JsonTemplate::Unsupported(),
            }
        } else {
            JsonTemplate::Unsupported()
        }
    }
}
