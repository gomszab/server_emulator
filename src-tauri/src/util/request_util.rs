use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tiny_http::Request;
use url::form_urlencoded;
use urlencoding::decode;

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
    pub request: Option<Value>
}

pub fn create_request_wrapper(endpoint: Endpoint, request:&mut Request) -> Option<RequestWrapper> {
    let method = request.method().to_string();
    let request_path = request.url().to_string();
    if endpoint.method != method {
        return None;
    }

    let (endpoint_base, endpoint_query) = split_path_and_query(&endpoint.path);
    let (request_base, request_query) = split_path_and_query(&request_path);

    // Match path segments with parameters
    let mut path_params = HashMap::new();
    let endpoint_segments: Vec<&str> = endpoint_base.trim_matches('/').split('/').collect();
    let request_segments: Vec<&str> = request_base.trim_matches('/').split('/').collect();

    if endpoint_segments.len() != request_segments.len() {
        return None;
    }

    for (e_seg, r_seg) in endpoint_segments.iter().zip(request_segments.iter()) {
        if e_seg.starts_with(':') {
            path_params.insert(e_seg[1..].to_string(), r_seg.to_string());
        } else if e_seg != r_seg {
            return None;
        }
    }

    // Handle query parameters
    let endpoint_q = parse_query(endpoint_query);
    let request_q = parse_query(request_query);



    // Case 1: Endpoint has no query params
    if endpoint_q.is_empty() {
        return if request_q.is_empty() {
            Some(RequestWrapper {
                path_params,
                query_params: request_q,
                endpoint,
                request: create_value_from_request(request)
            })
        } else {
            None
        };
    }

    if endpoint_q.keys().len().ne(&request_q.keys().len()) {
        return None;
    }

    let mut query_params = HashMap::new();
    for (key, e_val) in &endpoint_q {  
        if let Some(r_val) = request_q.get(key) {
            if e_val.starts_with("<") && e_val.ends_with(">") {
                let placeholder = e_val[1..e_val.len() - 1].to_string();
                let decoded = decode(r_val).expect("UTF-8");
                query_params.insert(placeholder, decoded.into_owned());
                
            } else if e_val != r_val {
                return None;
            }
        }
    }
    println!("{:?}", query_params);

    Some(RequestWrapper {
        path_params,
        query_params,
        endpoint,
        request: create_value_from_request(request)
    })
}

fn split_path_and_query(path: &str) -> (&str, Option<&str>) {
    let mut parts = path.splitn(2, '?');
    let base = parts.next().unwrap_or("").trim_end_matches('?');
    let query = parts.next();
    (base, query)
}

fn parse_query(query: Option<&str>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Some(q) = query.filter(|s| !s.is_empty()) {
        for pair in q.split('&') {
            let mut kv = pair.splitn(2, '=');
            let key = kv.next().unwrap_or_default().to_string();
            let value = kv.next().unwrap_or_default().to_string();
            params.insert(key, value);
        }
    }
    params
}

fn create_value_from_request( request:&mut Request) -> Option<Value>{
    if let Some(value) = request.headers().iter().find(|header| header.field.as_str() == "Content-Type") {
        let str = value.value.as_str(); 
        println!("{}",str);
        if str == "application/json" {
            let mut content = String::new();
            if let Some(_) = request.as_reader().read_to_string(&mut content).ok(){
               let k = serde_json::from_str(&content).ok();
               return k;
            }
        }else if str == "application/x-www-form-urlencoded"{
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).ok()?;
            let parsed: HashMap<_,_> = form_urlencoded::parse(content.as_bytes()).into_owned().collect();
            if let Some(parsed_json_string) = serde_json::to_string(&parsed).ok(){
                let k = serde_json::from_str(&parsed_json_string).ok();
                return k;
            }
        
        }   
    }
    None
}