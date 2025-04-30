use std::sync::Arc;

use crate::{
    html::{
        add_item_html, find_by_id_html, find_by_queryparam_html, no_logic_html, not_found_html,
        remove_by_id_html, return_dataset_html, HtmlResponse, HtmlTemplate,
    },
    json::{
        add_item_json, find_by_id_json, find_by_queryparam_json, not_found_json, remove_by_id_json,
        return_dataset_json, JsonResponse, JsonTemplate,
    },
    service::datasetstruct::DatasetFacade,
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
        facade: Arc<DatasetFacade>,
    ) -> Option<HtmlOrJson> {
        if let Some(wrapper) = create_request_wrapper(endpoint, request) {
            if &wrapper.endpoint.response_type == "json" {
                return Some(HtmlOrJson::JSON((facade, JsonTemplate::create(wrapper))));
            }
            if &wrapper.endpoint.response_type == "html" {
                return Some(HtmlOrJson::HTML((
                    facade,
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
            HtmlOrJson::HTML((facade, Some(htmlbody), htmltemplate)) => match htmltemplate {
                HtmlTemplate::ReturnDataSet() => return_dataset_html(facade, htmlbody),
                HtmlTemplate::AddItem(request) => add_item_html(facade, htmlbody, request),
                HtmlTemplate::FindById(pathparam) => find_by_id_html(facade, htmlbody, pathparam),
                HtmlTemplate::NoLogic(queryparam) => no_logic_html(htmlbody, queryparam),
                HtmlTemplate::FindByQueryParameter(queryparam) => {
                    find_by_queryparam_html(facade, queryparam, htmlbody)
                }
                HtmlTemplate::RemoveById(pathparam) => {
                    remove_by_id_html(facade, htmlbody, pathparam)
                }
                HtmlTemplate::NotFound() => not_found_html(),
            },
            HtmlOrJson::HTML((_, None, _)) => not_found_html(),
            HtmlOrJson::JSON((facade, jsontemplate)) => match jsontemplate {
                JsonTemplate::AddItem(request) => add_item_json(facade, request),
                JsonTemplate::FindById(pathparam) => find_by_id_json(facade, pathparam),
                JsonTemplate::RemoveById(pathparam) => remove_by_id_json(facade, pathparam),
                JsonTemplate::ReturnDataSet() => return_dataset_json(facade),
                JsonTemplate::Unsupported() => not_found_json(),
                JsonTemplate::FindByQueryParameter(queryparameter) => {
                    find_by_queryparam_json(facade, queryparameter)
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
