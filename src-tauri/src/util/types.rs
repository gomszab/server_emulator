use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use tiny_http::{Request, Response};

use super::{
    add_item_html, create_request_wrapper, find_by_id_html, find_by_queryparam, no_logic,
    not_found, return_dataset_html, Endpoint, RequestWrapper,
};

pub enum HtmlOrJson {
    HTML(Arc<Mutex<Vec<Value>>>, Option<String>, HtmlTemplate),
    JSON(Arc<Mutex<Vec<Value>>>, RequestWrapper),
}

pub enum HtmlTemplate {
    ReturnDataSet(),
    AddItem(Option<Value>),
    FindById(HashMap<String, String>),
    FindByQueryParameter(HashMap<String, String>),
    NoLogic(HashMap<String, String>),
    NotFound(),
}

impl HtmlTemplate {
    fn create(wrapper: RequestWrapper) -> HtmlTemplate {
        if let Some(logic) = &wrapper.endpoint.logic {
            match logic.as_str() {
                "add item" => HtmlTemplate::AddItem(wrapper.request),
                "return dataset" => HtmlTemplate::ReturnDataSet(),
                "find by id in dataset" => HtmlTemplate::FindById(wrapper.path_params),
                "find in dataset by queryparameter" => {
                    HtmlTemplate::FindByQueryParameter(wrapper.query_params)
                }
                _ => HtmlTemplate::NotFound(),
            }
        } else {
            HtmlTemplate::NoLogic(wrapper.query_params)
        }
    }
}

impl HtmlOrJson {
    pub fn create(
        endpoint: Endpoint,
        request: &mut Request,
        dataset: Arc<Mutex<Vec<Value>>>,
    ) -> Option<HtmlOrJson> {
        if let Some(wrapper) = create_request_wrapper(endpoint, request) {
            if &wrapper.endpoint.response_type == "json" {
                return Some(HtmlOrJson::JSON(dataset.clone(), wrapper));
            }
            if &wrapper.endpoint.response_type == "html" {
                return Some(HtmlOrJson::HTML(
                    dataset.clone(),
                    wrapper.endpoint.html_body.clone(),
                    HtmlTemplate::create(wrapper),
                ));
            }
        }
        None
    }
}

impl HtmlOrJson {
    pub fn to_response(&self) -> Response<std::io::Cursor<Vec<u8>>> {
        match self {
            HtmlOrJson::HTML(dataset, Some(htmlbody), htmltemplate) => match htmltemplate {
                HtmlTemplate::ReturnDataSet() => {
                    return_dataset_html(dataset.clone(), htmlbody.clone())
                }
                HtmlTemplate::AddItem(request) => {
                    add_item_html(dataset.clone(), htmlbody.clone(), request.clone())
                }
                HtmlTemplate::FindById(pathparam) => {
                    find_by_id_html(dataset.clone(), htmlbody.clone(), pathparam.clone())
                }
                HtmlTemplate::NoLogic(queryparam) => no_logic(htmlbody.clone(), queryparam.clone()),
                HtmlTemplate::FindByQueryParameter(queryparam) => {
                    find_by_queryparam(dataset.clone(), queryparam.clone(), htmlbody.clone())
                }
                HtmlTemplate::NotFound() => not_found(),
            },
            HtmlOrJson::HTML(_, None, _) => not_found(),
            HtmlOrJson::JSON(_dataset, _) => panic!("json things are not implemented yet"),
        }
    }
}

pub fn get_value_as_string(incomming_value: &Value) -> String {
    match incomming_value {
        Value::String(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        other => other.to_string(),
    }
}
