use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use tiny_http::Response;

use crate::service::{datasetstruct::DatasetFacade, read_operation::DatasetReadOperation, types::ElementNotFound};

use super::util::create_json_response;

pub fn find_by_id_json(
    facade: &Arc<DatasetFacade>,
    path_param: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {

    if let Some(id) = path_param.get("id") {
        match facade.get_by_id(id) {
            Ok(item) => create_json_response(&serde_json::to_string(&item).unwrap(), 200),
            Err(ElementNotFound) => create_json_response(r#"{"error": "Item not found"}"#, 404)
        }
    }else{
        create_json_response(r#"{"error": "Unrecognized id"}"#, 400)
    }
}
