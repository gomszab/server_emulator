use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde_json::Value;
use tiny_http::{Header, Response};

use crate::{html::template_engine, util::get_value_as_string};

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn find_by_queryparam_html(
    dataset: &Arc<Mutex<Vec<Value>>>,
    query_parameter: &HashMap<String, String>,
    template: &String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let data = dataset.lock().unwrap();

    if let Some(item) = data.iter().find(|item| {
        let mut counter = 0;
        let all = &query_parameter.len();
        for query_param in query_parameter {
            if get_value_as_string(&item[query_param.0]) == query_param.1.to_string() {
                counter += 1;
            }
        }
        return counter == *all;
    }) {
        let content = template_engine::render_template(
            &template,
            Some(item),
            None,
            Some(query_parameter.clone()),
            None,
        );
        Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
    } else {
        return make_invalid_response_html("<h1>Element not found</h1>");
    }
}