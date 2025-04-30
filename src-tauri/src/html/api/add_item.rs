use std::sync::Arc;

use serde_json::Value;
use tiny_http::{Header, Response};

use crate::{
    html::template_engine,
    service::{
        datasetstruct::DatasetFacade, types::InvalidRequest, write_operation::DatasetWriteOperation,
    },
};

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn add_item_html(
    facade: &Arc<DatasetFacade>,
    template: &String,
    request: &Option<Value>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = if let Some(request_body) = request {
        match facade.add_item(request_body.clone()) {
            Ok(dataset) => Ok(template_engine::render_template(
                template.as_str(),
                Some(dataset),
                None,
                None,
                Some(request_body.clone()),
            )),
            Err(InvalidRequest) => Err("<h1>Invalid request</h1>"),
        }
    } else {
        Err("<h1>Empty request</h1>")
    };
    match content {
        Ok(content) => {
            Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        }
        Err(message) => make_invalid_response_html(message),
    }
}
