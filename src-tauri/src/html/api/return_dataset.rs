use std::sync::Arc;

use tiny_http::{Header, Response};

use crate::{
    html::template_engine,
    service::{datasetstruct::DatasetFacade, read_operation::DatasetReadOperation},
};

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn return_dataset_html(
    facade: &Arc<DatasetFacade>,
    template: &String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = match facade.get_all() {
        Ok(dataset) => Ok(template_engine::render_template(
            template.as_str(),
            Some(dataset),
            None,
            None,
            None,
        )),
        Err(_) => Err("Something went wrong"),
    };
    match content {
        Ok(content) => {
            Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        }
        Err(message) => make_invalid_response_html(message),
    }
}
