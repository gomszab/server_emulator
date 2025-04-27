use std::collections::HashMap;

use serde_json::Value;
use tiny_http::Request;

use crate::util::Endpoint;

use super::{method_validator::MethodValidator, path_validator::PathValidator, query_validator::QueryValidator, request_parser::RequestParser};

pub(crate) trait RequestFilter {
    fn do_filter(&self, context: &mut ValidationContext) -> Option<()>;
}

pub struct ValidationContext<'a> {
    pub endpoint: Endpoint,
    pub request: &'a mut Request,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub parsed_request: Option<Value>
}

impl<'a> ValidationContext<'a> {
   pub fn new(endpoint: Endpoint, request: &'a mut Request) -> Self {
        Self {
            endpoint,
            request,
            path_params: HashMap::new(),
            query_params: HashMap::new(),
            parsed_request: None
        }
    }
}

pub struct FilterChain {
    filter_list: Vec<Box<dyn RequestFilter>>,
}

impl FilterChain {
    pub fn new() -> Self {
        Self {
            filter_list: vec![
                Box::new(MethodValidator),
                Box::new(PathValidator),
                Box::new(QueryValidator),
                Box::new(RequestParser)
            ],
        }
    }

   pub fn execute(&self, context: &mut ValidationContext) -> Option<()> {
        for filter in &self.filter_list {
            if filter.do_filter(context).is_none() {
                return None;
            }
        }
        Some(())
    }
}