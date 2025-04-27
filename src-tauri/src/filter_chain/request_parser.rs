use std::collections::HashMap;

use serde_json::Value;
use tiny_http::Request;
use url::form_urlencoded;

use super::chain::RequestFilter;

pub(crate) struct RequestParser;

impl RequestFilter for RequestParser{
    fn do_filter(&self, ctx: &mut super::chain::ValidationContext) -> Option<()> {
        ctx.parsed_request = create_value_from_request(ctx.request);
        Some(())
    }
}

fn create_value_from_request( request:&mut Request) -> Option<Value>{
    if let Some(value) = request.headers().iter().find(|header| header.field.as_str() == "Content-Type") {
        let str = value.value.as_str(); 
        println!("Content-Type: {}",str);
        if str == "application/json" {
            let mut content = String::new();
            if let Some(_) = request.as_reader().read_to_string(&mut content).ok(){
               let k = serde_json::from_str(&content).ok();
               println!("Body: {}",content);
               return k;
            }
        }else if str == "application/x-www-form-urlencoded"{
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).ok()?;
            let parsed: HashMap<_,_> = form_urlencoded::parse(content.as_bytes()).into_owned().collect();
            if let Some(parsed_json_string) = serde_json::to_string(&parsed).ok(){
                let k = serde_json::from_str(&parsed_json_string).ok();
                println!("Body: {}",parsed_json_string);
                return k;
            }
        
        }   
    }
    None
}