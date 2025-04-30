use std::{collections::HashMap, sync::Arc};

use tiny_http::{Header, Response};

use crate::{
    html::template_engine,
    service::{
        datasetstruct::DatasetFacade, read_operation::DatasetReadOperation, types::ElementNotFound,
    },
};

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn find_by_queryparam_html(
    facade: &Arc<DatasetFacade>,
    query_parameter: &HashMap<String, String>,
    template: &String,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = match facade.get_by_parameters(query_parameter) {
        Ok(item) => Ok(template_engine::render_template(
            &template,
            Some(&item),
            None,
            Some(query_parameter.clone()),
            None,
        )),
        Err(ElementNotFound) => Err("<h1>Element Not Found</h1>"),
    };

    match content {
        Ok(content) => {
            Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        }
        Err(message) => make_invalid_response_html(message),
    }
}
