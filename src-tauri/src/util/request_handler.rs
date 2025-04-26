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
        if let Some(result) = HtmlOrJson::create(endpoint.clone(), request , dataset.clone()) {
            return result.to_response();
            // return match result.endpoint.response_type.as_str() {
            //     "json" => handle_json_request(&result, request, &dataset),
            //     "html" => handle_html_request(&result, request, &dataset, &url),
            //     _ => Response::from_string("Invalid response type").with_status_code(500),
            // };   
        }
    }
    
    Response::from_string("Not found").with_status_code(404)
}







// fn handle_json_request(
//     match_result: &RequestWrapper,
//     request: &mut Request,
//     dataset: &Arc<Mutex<Vec<Value>>>,
// ) -> Response<std::io::Cursor<Vec<u8>>> {
//     if let Some(logic) = &match_result.endpoint.logic {
//         return match logic.as_str() {
//             // Return the entire dataset
//             "return dataset" => {
//                 if let Ok(data) = dataset.lock(){
//                     if let Ok(json) = serde_json::to_string(&*data) {
//                         Response::from_string(json)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                     }else{
//                         Response::from_string(r#"{"message": "Something wrong"}"#)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                     }
//                 }else{
//                     Response::from_string(r#"{"message": "Something wrong"}"#)
//                     .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                 }
                
//             }

//             "add item" => {
//                 let mut data = dataset.lock().unwrap();
               
//                 if let Some(mut json_value) =  create_value_from_request(request){
//                     if let Value::Object(ref mut map) = json_value {
//                         map.insert("id".to_string(), json!((data.len()+1) as u64));
//                         data.push(json_value.take());
//                         Response::from_string(r#"{"message": "Item added"}"#)
//                     .with_header::<Header>("Content-Type: application/json".parse().unwrap()).with_status_code(201)
//                     } else {
//                         Response::from_string(r#"{"message": "Invalid json"}"#)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap()).with_status_code(503)
//                     }
//                 }else{
//                     Response::from_string(r#"{"message": "cannot be parsed as json"}"#)
//                      .with_header::<Header>("Content-Type: application/json".parse().unwrap()).with_status_code(503)
//                 }
//             }
    
//             // Find item by ID in the dataset
//             "find by id in dataset" => {
//                 let id =  match_result.path_params.get("id")
//                     .and_then(|id| id.parse::<String>().ok());
//                 let data = dataset.lock().unwrap();
//                 if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"].to_string() == id)) {
//                     let json = serde_json::to_string(item).unwrap();
//                     Response::from_string(json)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                 } else { 
//                     Response::from_string(r#"{"error": "Item not found"}"#)
//                         .with_status_code(404)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                 }
//             }
    
//             // Remove item by ID from the dataset
//             "remove by id in dataset" => {
//                 let id = match_result.path_params.get("id");
    
//                 let mut data = dataset.lock().unwrap();
//                 if let Some(pos) = id.and_then(|id| data.iter().position(|item| item["id"].to_string() == *id)) {
//                     data.remove(pos);
//                     Response::from_string(r#"{"status": "success"}"#)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                 } else {
//                     Response::from_string(r#"{"error": "Item not found"}"#)
//                         .with_status_code(404)
//                         .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//                 }
//             }
    
//             // Unsupported logic
//             _ => Response::from_string(r#"{"error": "Unsupported action"}"#)
//                 .with_status_code(400)
//                 .with_header::<Header>("Content-Type: application/json".parse().unwrap()),
//         }
//     }else{
//         Response::from_string(r#"{"error": "Unsupported action"}"#)
//             .with_status_code(400)
//             .with_header::<Header>("Content-Type: application/json".parse().unwrap())
//     }

   
// }

// fn handle_html_request(
//     match_result: &RequestWrapper,
//     request: &mut Request,
//     dataset: &Arc<Mutex<Vec<Value>>>,
//     url: &str,
// ) -> Response<std::io::Cursor<Vec<u8>>> {
//     let route_params = &match_result.query_params;
//     let query_params = &match_result.query_params;
//     let mut request_body = create_value_from_request(request);

//     let content = template_engine::render_template(
//         match_result.endpoint.html_body.as_deref().unwrap_or_default(),
//         dataset,
//         &route_params,
//         &query_params,
//         &request_body,
//     );

//     if let Some(logic) = &match_result.endpoint.logic {
//         return match logic.as_str() {
//             "add item" => {
//                 {
//                     let mut data = dataset.lock().unwrap();
//                     if let Some(ref mut value) =  request_body {
//                         if let Value::Object(ref mut map) = value {
//                             // Convert data.len() to JSON-compatible number
//                             map.insert("id".to_string(), json!((data.len()+1) as u64));
//                             data.push(value.clone());
//                         } else {
//                             return Response::from_string("<h1>Invalid request body</h1>")
//                             .with_header::<Header>(HTML_RESPONSE.parse().unwrap());
//                         }
//                     }else{
//                         return Response::from_string("<h1>Invalid request body</h1>")
//                         .with_header::<Header>(HTML_RESPONSE.parse().unwrap());
//                     }       
                    
//                 }
//                 let content = template_engine::render_template(
//                     match_result.endpoint.html_body.as_deref().unwrap_or_default(),
//                     dataset,
//                     &route_params,
//                     &query_params,
//                     &request_body,
//                 );
//                 Response::from_string(content)
//                 .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//             },
//             "return dataset" => {
//                 let content = template_engine::render_template(
//                     match_result.endpoint.html_body.as_deref().unwrap_or_default(),
//                     dataset,
//                     &route_params,
//                     &query_params,
//                     &request_body,
//                 );
//                 Response::from_string(content)
//                 .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//             },
//             "find by id in dataset" => {
//                 let id = web_utils::extract_route_parameter(&match_result.endpoint.path, request.url(), "id")
//                 .and_then(|id| id.parse::<String>().ok());
//                 let data = dataset.lock().unwrap();
                
//                 if let Some(item) = id.and_then(|id| data.iter().find(|item| item["id"].to_string() == id)) {
//                     let content = template_engine::render_template(
//                         match_result.endpoint.html_body.as_deref().unwrap_or_default(),
//                         item,
//                         &route_params,
//                         &query_params,
//                         &request_body,
//                     );
//                     Response::from_string(content)
//                 .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//                 }else{
//                     Response::from_string(content)
//                 .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//                 }
//             }
//             "find in dataset by queryparameter" => {
//                 let data = dataset.lock().unwrap();
                
//                 if let Some(item) = data.iter().find(|item| {
//                     let mut counter = 0;
//                     let all = &query_params.len();
//                     for query_param in query_params{

//                         if get_value_as_string(&item[query_param.0]) == query_param.1.to_string(){
//                             counter+=1; 
//                         }
                        
//                     }
//                     println!("{}, {}", counter, all);
//                     return counter == *all;
//                 }) {
//                     let content = template_engine::render_template(
//                         match_result.endpoint.html_body.as_deref().unwrap_or_default(),
//                         item,
//                         &route_params,
//                         &query_params,
//                         &request_body,
//                     );
//                     Response::from_string(content)
//                 .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//                 }else{
//                     Response::from_string("Az elem nem található a megadott paraméterekkel.")
//                 .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//                 }
//             }
//             _ => {
//                 Response::from_string(content)
//         .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//             }
//         }
//     }else{
//        return  Response::from_string(content)
//         .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
//     }
// }


