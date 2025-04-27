use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;

use crate::util::RequestWrapper;

pub type HtmlResponse = (Arc<Mutex<Vec<Value>>>, Option<String>, HtmlTemplate);

pub enum HtmlTemplate {
    ReturnDataSet(),
    AddItem(Option<Value>),
    FindById(HashMap<String, String>),
    FindByQueryParameter(HashMap<String, String>),
    RemoveById(HashMap<String, String>),
    NoLogic(HashMap<String, String>),
    NotFound(),
}

impl HtmlTemplate {
    pub fn create(wrapper: RequestWrapper) -> HtmlTemplate {
        if let Some(logic) = &wrapper.endpoint.logic {
            match logic.as_str() {
                "add item" => HtmlTemplate::AddItem(wrapper.request),
                "return dataset" => HtmlTemplate::ReturnDataSet(),
                "find by id in dataset" => HtmlTemplate::FindById(wrapper.path_params),
                "find in dataset by queryparameter" => {
                    HtmlTemplate::FindByQueryParameter(wrapper.query_params)
                },
                "remove by id in dataset" => HtmlTemplate::RemoveById(wrapper.path_params),
                _ => HtmlTemplate::NotFound(),
            }
        } else {
            HtmlTemplate::NoLogic(wrapper.query_params)
        }
    }
}
