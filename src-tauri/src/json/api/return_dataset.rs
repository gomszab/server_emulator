use std::sync::Arc;

use tiny_http::Response;

use crate::service::{datasetstruct::DatasetFacade, read_operation::DatasetReadOperation};

use super::util::create_json_response;

pub fn return_dataset_json(facade: &Arc<DatasetFacade>) -> Response<std::io::Cursor<Vec<u8>>> {
    match facade.get_all() {
        Ok(dataset) => create_json_response(&serde_json::to_string(dataset).unwrap(), 200),
        Err(_) => create_json_response(r#"{"message": "Something wrong"}"#, 400),
    }
}
