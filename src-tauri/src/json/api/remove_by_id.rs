use std::{collections::HashMap, sync::Arc};

use tiny_http::Response;

use crate::service::{datasetstruct::DatasetFacade, write_operation::DatasetWriteOperation};

use super::util::create_json_response;

pub fn remove_by_id_json(
    facade: &Arc<DatasetFacade>,
    path_param: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    if let Some(id) = path_param.get("id") {
        match facade.remove_by_id(id.clone()) {
            Ok(_) => create_json_response(r#"{"error": "Item removed"}"#, 200),
            Err(_) => create_json_response(r#"{"error": "Item not found"}"#, 404),
        }
    } else {
        create_json_response(r#"{"error": "Unrecognized id"}"#, 404)
    }
}
