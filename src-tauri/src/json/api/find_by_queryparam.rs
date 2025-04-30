use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde_json::Value;
use tiny_http::Response;

use crate::{service::{datasetstruct::DatasetFacade, read_operation::DatasetReadOperation, types::ElementNotFound}, util::get_value_as_string};

use super::util::create_json_response;

pub fn find_by_queryparam_json(
    facade: &Arc<DatasetFacade>,
    query_parameter: &HashMap<String, String>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    
    match facade.get_by_parameters(query_parameter) {
        Ok(item) =>  create_json_response(&serde_json::to_string(&item).unwrap(), 200),
        Err(ElementNotFound) => create_json_response(r#"{"error": "Item not found"}"#, 404)
    }
}