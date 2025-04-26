use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde_json::{json, Value};
use tiny_http::{Header, Response};

use crate::util::get_value_as_string;

use super::template_engine;

const HTML_RESPONSE: &str = "Content-Type: text/html; charset=utf-8";

pub fn make_invalid_response_html(message: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(message)
        .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        .with_status_code(503)
}

pub fn return_dataset_html(
    dataset: Arc<Mutex<Vec<Value>>>,
    template: String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content =
        template_engine::render_template(template.as_str(), Some(&dataset), None, None, None);
    Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
}

pub fn add_item_html(
    dataset: Arc<Mutex<Vec<Value>>>,
    template: String,
    mut request_body: Option<Value>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    {
        let mut data = dataset.lock().unwrap();
        if let Some(ref mut value) = request_body {
            if let Value::Object(ref mut map) = value {
                // Convert data.len() to JSON-compatible number
                map.insert("id".to_string(), json!((data.len() + 1) as u64));
                data.push(value.clone());
            } else {
                return make_invalid_response_html("<h1>Invalid request body</h1>");
            }
        } else {
            return make_invalid_response_html("<h1>Invalid request body</h1>");
        }
    }

    let content =
        template_engine::render_template(template.as_str(), Some(&dataset), None, None, request_body);
    Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
}

pub fn find_by_id_html(
    dataset: Arc<Mutex<Vec<Value>>>,
    template: String,
    path_param: HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let id = path_param.get("id");
    let data = dataset.lock().unwrap();

    if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"].to_string() == *id)) {
        let content = template_engine::render_template(
            template.as_str(),
            Some(item),
            Some(path_param),
            None,
            None,
        );
        Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
    } else {
        return make_invalid_response_html("<h1>Invalid id</h1>");
    }
}

pub fn no_logic(
    template: String,
    query_parameter: HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = template_engine::render_template::<&Value>(
        template.as_str(),
        None,
        None,
        Some(query_parameter),
        None,
    );
    Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
}

pub fn find_by_queryparam(
    dataset: Arc<Mutex<Vec<Value>>>,
    query_parameter: HashMap<String, String>,
    template: String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let data = dataset.lock().unwrap();

    if let Some(item) = data.iter().find(|item| {
        let mut counter = 0;
        let all = &query_parameter.len();
        for query_param in &query_parameter {
            if get_value_as_string(&item[query_param.0]) == query_param.1.to_string() {
                counter += 1;
            }
        }
        println!("{}, {}", counter, all);
        return counter == *all;
    }) {
        let content = template_engine::render_template(
            &template,
            Some(item),
            None,
            Some(query_parameter),
            None,
        );
        Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
    } else {
        return make_invalid_response_html("<h1>Element not found</h1>");
    }
}

pub fn not_found() -> Response<std::io::Cursor<Vec<u8>>>{
    return make_invalid_response_html("<h1>Page not found</h1>");
}