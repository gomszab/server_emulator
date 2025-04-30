use std::{collections::HashMap, sync::Arc};

use tiny_http::{Header, Response};

use crate::{
    html::template_engine,
    service::{
        datasetstruct::DatasetFacade, read_operation::DatasetReadOperation, types::ElementNotFound,
    },
};

use super::util::{make_invalid_response_html, HTML_RESPONSE};

pub fn find_by_id_html(
    facade: &Arc<DatasetFacade>,
    template: &String,
    path_param: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let content = if let Some(id) = path_param.get("id") {
        match facade.get_by_id(id) {
            Ok(item) => Ok(template_engine::render_template(
                template.as_str(),
                Some(&item),
                Some(path_param.clone()),
                None,
                None,
            )),
            Err(ElementNotFound) => Err("<h1>Invalid id</h1>"),
        }
    } else {
        Err("Unrecognized id parameter")
    };

    match content {
        Ok(content) => {
            Response::from_string(content).with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        }
        Err(message) => make_invalid_response_html(message),
    }
}
