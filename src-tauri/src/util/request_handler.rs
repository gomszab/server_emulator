use crate::service::datasetstruct::DatasetFacade;

use super::{Endpoint, HtmlOrJson};
use std::sync::Arc;
use tiny_http::{Request, Response};

pub fn handle_request(
    request: &mut Request,
    endpoints: &[Endpoint],
    facade: Arc<DatasetFacade>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    for endpoint in endpoints {
        if let Some(result) = HtmlOrJson::create(endpoint.clone(), request, facade.clone()) {
            return result.to_response();
        }
    }

    Response::from_string("Not found").with_status_code(404)
}
