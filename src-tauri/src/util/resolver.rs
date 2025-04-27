use std::sync::{Arc, Mutex};

use crate::{
    html::{
        add_item_html, find_by_id_html, find_by_queryparam_html, no_logic_html, not_found_html, remove_by_id_html, return_dataset_html, HtmlResponse, HtmlTemplate
    },
    json::{
        add_item_json, find_by_id_json, find_by_queryparam_json, not_found_json, remove_by_id_json,
        return_dataset_json, JsonResponse, JsonTemplate,
    },
};
use serde_json::Value;
use tiny_http::{Request, Response};

use super::{create_request_wrapper, Endpoint};

pub enum HtmlOrJson {
    HTML(HtmlResponse),
    JSON(JsonResponse),
}

impl HtmlOrJson {
    pub fn create(
        endpoint: Endpoint,
        request: &mut Request,
        dataset: Arc<Mutex<Vec<Value>>>,
    ) -> Option<HtmlOrJson> {
        if let Some(wrapper) = create_request_wrapper(endpoint, request) {
            if &wrapper.endpoint.response_type == "json" {
                return Some(HtmlOrJson::JSON((
                    dataset.clone(),
                    JsonTemplate::create(wrapper),
                )));
            }
            if &wrapper.endpoint.response_type == "html" {
                return Some(HtmlOrJson::HTML((
                    dataset.clone(),
                    wrapper.endpoint.html_body.clone(),
                    HtmlTemplate::create(wrapper),
                )));
            }
        }
        None
    }
}

impl HtmlOrJson {
    pub fn to_response(&self) -> Response<std::io::Cursor<Vec<u8>>> {
        match self {
            HtmlOrJson::HTML((dataset, Some(htmlbody), htmltemplate)) => match htmltemplate {
                HtmlTemplate::ReturnDataSet() => return_dataset_html(dataset, htmlbody),
                HtmlTemplate::AddItem(request) => add_item_html(dataset, htmlbody, request),
                HtmlTemplate::FindById(pathparam) => find_by_id_html(dataset, htmlbody, pathparam),
                HtmlTemplate::NoLogic(queryparam) => no_logic_html(htmlbody, queryparam),
                HtmlTemplate::FindByQueryParameter(queryparam) => {
                    find_by_queryparam_html(dataset, queryparam, htmlbody)
                },
                HtmlTemplate::RemoveById(pathparam) => remove_by_id_html(dataset, htmlbody, pathparam),
                HtmlTemplate::NotFound() => not_found_html(),
            },
            HtmlOrJson::HTML((_, None, _)) => not_found_html(),
            HtmlOrJson::JSON((dataset, jsontemplate)) => match jsontemplate {
                JsonTemplate::AddItem(request) => add_item_json(dataset, request),
                JsonTemplate::FindById(pathparam) => find_by_id_json(dataset, pathparam),
                JsonTemplate::RemoveById(pathparam) => remove_by_id_json(dataset, pathparam),
                JsonTemplate::ReturnDataSet() => return_dataset_json(dataset),
                JsonTemplate::Unsupported() => not_found_json(),
                JsonTemplate::FindByQueryParameter(queryparameter) => {
                    find_by_queryparam_json(dataset, queryparameter)
                }
            },
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
