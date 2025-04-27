use super::{Endpoint, HtmlOrJson};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tiny_http::{Request, Response};

pub fn handle_request(
    request: &mut Request,
    endpoints: &[Endpoint],
    dataset: Arc<Mutex<Vec<Value>>>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    for endpoint in endpoints {
        if let Some(result) = HtmlOrJson::create(endpoint.clone(), request, dataset.clone()) {
            return result.to_response();
        }
    }

    Response::from_string("Not found").with_status_code(404)
}
