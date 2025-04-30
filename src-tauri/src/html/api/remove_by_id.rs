use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use tiny_http::{Header, Response};

use crate::{
    html::template_engine,
    service::{
        datasetstruct::DatasetFacade, types::ElementNotFound,
        write_operation::DatasetWriteOperation,
    },
};

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn remove_by_id_html(
    facade: &Arc<DatasetFacade>,
    template: &str,
    path_param: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = if let Some(id) = path_param.get("id") {
        match facade.remove_by_id(id.to_string()) {
            Ok(dataset) => Ok(template_engine::render_template(
                template,
                Some(dataset),
                Some(path_param.clone()),
                None,
                None,
            )),
            Err(ElementNotFound) => Err("<h1>Element not found</h1>"),
        }
    } else {
        Err("<h1>Unrecognized id parameter</h1>")
    };

    match content {
        Ok(content) => {
            Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        }
        Err(message) => make_invalid_response_html(message),
    }
}
