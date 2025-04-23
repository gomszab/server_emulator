use super::{web_utils, template_engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tiny_http::{Header, Request, Response};
use url::form_urlencoded;

#[derive(Deserialize, Serialize)]
pub struct Endpoint {
    pub method: String,
    pub path: String,
    pub response_type: String,
    pub logic: Option<String>,
    pub html_body: Option<String>,
}

pub fn handle_request(
    request: &mut Request,
    endpoints: &[Endpoint],
    dataset: Arc<Mutex<Vec<Value>>>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let method = request.method().to_string();
    let url = request.url().to_string();
    
    for endpoint in endpoints {
        if endpoint.method == method && path_matches(&endpoint.path, &url) {
            return match endpoint.response_type.as_str() {
                "json" => handle_json_request(&endpoint, request, &dataset),
                "html" => handle_html_request(&endpoint, request, &dataset, &url),
                _ => Response::from_string("Invalid response type").with_status_code(500),
            };   
        }
    }
    
    Response::from_string("Not found").with_status_code(404)
}

fn path_matches(endpoint_path: &str, request_path: &str) -> bool {
    let endpoint_segments: Vec<&str> = endpoint_path.split('/').collect();
    let request_segments: Vec<&str> = request_path.split('/').collect();
    endpoint_segments.len() == request_segments.len() && 
    endpoint_segments.iter().zip(request_segments.iter()).all(|(a, b)| {
        a.starts_with(':') || *a == b.split('?').nth(0).unwrap()
    })
}

fn handle_json_request(
    endpoint: &super::Endpoint,
    request: &mut Request,
    dataset: &Arc<Mutex<Vec<Value>>>,
) -> Response<std::io::Cursor<Vec<u8>>> {
    if let Some(logic) = &endpoint.logic {
        return match logic.as_str() {
            // Return the entire dataset
            "return dataset" => {
                let data = dataset.lock().unwrap();
                let json = serde_json::to_string(&*data).unwrap();
                Response::from_string(json)
                    .with_header::<Header>("Content-Type: application/json".parse().unwrap())
            }

            "add item" => {
                let mut data = dataset.lock().unwrap();
                let mut content = String::new();
                request.as_reader().read_to_string(&mut content).ok();
                let mut value = serde_json::from_str(&content).ok().expect("invalid json");
                if let Value::Object(ref mut map) = value {
                    // Convert data.len() to JSON-compatible number
                    map.insert("id".to_string(), json!((data.len()+1) as u64));
                } else {
                    panic!("Expected JSON object");
                }
                data.push(value);
                Response::from_string(r#"{"message": "Item added"}"#)
                    .with_header::<Header>("Content-Type: application/json".parse().unwrap()).with_status_code(201)
            }
    
            // Find item by ID in the dataset
            "find by id in dataset" => {
                let id = web_utils::extract_route_parameter(&endpoint.path, request.url(), "id")
                    .and_then(|id| id.parse::<String>().ok());
                println!("{:?}", id);
                let data = dataset.lock().unwrap();
                if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"] == id)) {
                    let json = serde_json::to_string(item).unwrap();
                    Response::from_string(json)
                        .with_header::<Header>("Content-Type: application/json".parse().unwrap())
                } else {
                    Response::from_string(r#"{"error": "Item not found"}"#)
                        .with_status_code(404)
                        .with_header::<Header>("Content-Type: application/json".parse().unwrap())
                }
            }
    
            // Remove item by ID from the dataset
            "remove by id in dataset" => {
                let id = web_utils::extract_route_parameter(&endpoint.path, request.url(), "id")
                    .and_then(|id| id.parse::<String>().ok());
    
                let mut data = dataset.lock().unwrap();
                if let Some(pos) = id.and_then(|id| data.iter().position(|item| item["id"] == id)) {
                    data.remove(pos);
                    Response::from_string(r#"{"status": "success"}"#)
                        .with_header::<Header>("Content-Type: application/json".parse().unwrap())
                } else {
                    Response::from_string(r#"{"error": "Item not found"}"#)
                        .with_status_code(404)
                        .with_header::<Header>("Content-Type: application/json".parse().unwrap())
                }
            }
    
            // Unsupported logic
            _ => Response::from_string(r#"{"error": "Unsupported action"}"#)
                .with_status_code(400)
                .with_header::<Header>("Content-Type: application/json".parse().unwrap()),
        }
    }else{
        Response::from_string(r#"{"error": "Unsupported action"}"#)
            .with_status_code(400)
            .with_header::<Header>("Content-Type: application/json".parse().unwrap())
    }

   
}

fn handle_html_request(
    endpoint: &Endpoint,
    request: &mut Request,
    dataset: &Arc<Mutex<Vec<Value>>>,
    url: &str,
) -> Response<std::io::Cursor<Vec<u8>>> {
    let route_params = extract_route_params(&endpoint.path, url);
    let query_params = extract_query_params(url);
    let request_body = read_request_body(request);

    let content = template_engine::render_template(
        endpoint.html_body.as_deref().unwrap_or_default(),
        dataset,
        &route_params,
        &query_params,
        request_body.as_deref(),
    );

    if let Some(logic) = &endpoint.logic {
        return match logic.as_str() {
            "add item" => {
                {
                    let mut data = dataset.lock().unwrap();
                    let mut value = serde_json::from_str(&request_body.clone().unwrap()).ok().expect("invalid json");
                    if let Value::Object(ref mut map) = value {
                        // Convert data.len() to JSON-compatible number
                        map.insert("id".to_string(), json!((data.len()+1) as u64));
                    } else {
                        panic!("Expected JSON object");
                    }
                    data.push(value);
                }
                let content = template_engine::render_template(
                    endpoint.html_body.as_deref().unwrap_or_default(),
                    dataset,
                    &route_params,
                    &query_params,
                    request_body.as_deref(),
                );
                Response::from_string(content)
                .with_header::<Header>("Content-Type: text/html".parse().unwrap())
            },
            "return dataset" => {
                let content = template_engine::render_template(
                    endpoint.html_body.as_deref().unwrap_or_default(),
                    dataset,
                    &route_params,
                    &query_params,
                    request_body.as_deref(),
                );
                Response::from_string(content)
                .with_header::<Header>("Content-Type: text/html".parse().unwrap())
            },
            "find by id in dataset" => {
                let id = web_utils::extract_route_parameter(&endpoint.path, request.url(), "id")
                .and_then(|id| id.parse::<String>().ok());
                let data = dataset.lock().unwrap();
                
                if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"] == id)) {
                    let content = template_engine::render_template(
                        endpoint.html_body.as_deref().unwrap_or_default(),
                        item,
                        &route_params,
                        &query_params,
                        request_body.as_deref(),
                    );
                    Response::from_string(content)
                .with_header::<Header>("Content-Type: text/html".parse().unwrap())
                }else{
                    Response::from_string(content)
                .with_header::<Header>("Content-Type: text/html".parse().unwrap())
                }
                
            
            }
            _ => {
                Response::from_string(content)
        .with_header::<Header>("Content-Type: text/html".parse().unwrap())
            }
        }
    }else{
       return  Response::from_string(content)
        .with_header::<Header>("Content-Type: text/html".parse().unwrap())
    }
}

// Helper functions for param extraction
fn extract_route_params<'a>(endpoint_path: &'a str, url: &'a str) -> Vec<(&'a str, String)> {
    endpoint_path.split('/')
        .filter(|s| s.starts_with(':'))
        .filter_map(|param| {
            let param_name = &param[1..];
            web_utils::extract_route_parameter(endpoint_path, url, param_name)
                .map(|v| (param_name, v))
        })
        .collect()
}

fn extract_query_params(url: &str) -> Vec<(&str, String)> {
    url.split('?').nth(1).unwrap_or_default()
        .split('&')
        .filter_map(|pair| {
            let mut kv = pair.splitn(2, '=');
            match (kv.next(), kv.next()) {
                (Some(k), Some(v)) => Some((k, v.to_string())),
                _ => None,
            }
        })
        .collect()
}

fn read_request_body(request: &mut Request) -> Option<String> {
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).ok()?;
    let parsed: HashMap<_,_> = form_urlencoded::parse(content.as_bytes()).into_owned().collect();
    serde_json::to_string(&parsed).ok()
}
