use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tiny_http::Request;

use crate::filter_chain::chain::{FilterChain, ValidationContext};

#[derive(Deserialize, Serialize, Clone)]
pub struct Endpoint {
    pub method: String,
    pub path: String,
    pub response_type: String,
    pub logic: Option<String>,
    pub html_body: Option<String>,
}

pub struct RequestWrapper {
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub endpoint: Endpoint,
    pub request: Option<Value>,
}

pub fn create_request_wrapper(endpoint: Endpoint, request: &mut Request) -> Option<RequestWrapper> {
    let mut context = ValidationContext::new(endpoint, request);
    let filter_chain = FilterChain::new();
    filter_chain.execute(&mut context)?;

    Some(RequestWrapper {
        path_params: context.path_params,
        query_params: context.query_params,
        endpoint: context.endpoint,
        request: context.parsed_request,
    })
}
