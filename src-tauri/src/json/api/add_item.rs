use std::sync::Arc;

use serde_json::Value;
use tiny_http::Response;

use crate::service::{datasetstruct::DatasetFacade, write_operation::DatasetWriteOperation};

use super::util::create_json_response;

pub fn add_item_json(
    facade: &Arc<DatasetFacade>,
    request: &Option<Value>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    if let Some(request_body) = request {
        match facade.add_item(request_body.clone()) {
            Ok(_) => create_json_response(r#"{"message": "Item added"}"#, 201),
            Err(_) => create_json_response(r#"{"message": "Invalid json"}"#, 400),
        }
    } else {
        create_json_response(r#"{"message": "Invalid json"}"#, 400)
    }
}
