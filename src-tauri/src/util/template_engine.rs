use serde_json::Value;
use std::sync::{Arc, Mutex};

enum DataSetOrSingleValue<'a>{
    Dataset(&'a Arc<Mutex<Vec<Value>>>),
    SingleValue(&'a Value)
}

trait DataSetOrSingleValueTrait {
    fn to_dataset_single(&self) -> DataSetOrSingleValue;
}

impl DataSetOrSingleValueTrait for &Arc<Mutex<Vec<Value>>> {
    fn to_dataset_single(&self) -> DataSetOrSingleValue {
        DataSetOrSingleValue::Dataset(*self)
    }
}

impl DataSetOrSingleValueTrait for &Value {
    fn to_dataset_single(&self) -> DataSetOrSingleValue {
        DataSetOrSingleValue::SingleValue(*self)
    }
}

#[allow(private_bounds)]
pub fn render_template<T: DataSetOrSingleValueTrait>(
    template: &str,
    datasetparam: T,
    route_params: &[(&str, String)],
    query_params: &[(&str, String)],
    request_body: Option<&str>,
) -> String {
    let mut output = template.to_string();
    
    // Handle dataset replacement
    if output.contains("{memory:dataset}") {
        if let DataSetOrSingleValue::Dataset(dataset) = &datasetparam.to_dataset_single() {
            let mut dataset = dataset.lock().unwrap();
            let mappedrows: Vec<String> = dataset.iter_mut().map(|element| {
                let mut result = String::new();
                if let Value::Object(ref mut map) = element {
                   
                    result.push_str("<tr>");
                    for pair in map  {
                        if let Some(raw_value) = pair.1.as_str() {
                            result.push_str(format!("<td>{}</td>", raw_value).as_str());
                        } else {
                            // Fallback for non-string values
                            result.push_str(format!("<td>{}</td>", pair.1.to_string()).as_str());
                        }
                    }
                    result.push_str("</tr>");
                    
                }
                result
            }).collect();
            output = output.replace(
                "{memory:dataset}", 
                &format!("{}", mappedrows.join(""))
            );
        }
        
    }

    if output.contains("{memory:element}") {
        if let DataSetOrSingleValue::SingleValue(element) = &datasetparam.to_dataset_single() {
            let mut result = String::new();
            if let Value::Object(map) = element{
                for pair in map  {
                    if let Some(raw_value) = pair.1.as_str() {
                        result.push_str(format!("<h1>{}</h1>", raw_value).as_str());
                    } else {
                        // Fallback for non-string values
                        result.push_str(format!("<h1>{}</h1>", pair.1.to_string()).as_str());
                    }
                } 
                output = output.replace(
                    "{memory:element}", 
                    &format!("{}", result)
                );
            }
        }
        
    }

    // Replace route parameters
    for (key, value) in route_params {
        output = output.replace(&format!("{{routeparam:{}}}", key), value);
    }

    // Replace query parameters
    for (key, value) in query_params {
        output = output.replace(&format!("{{queryparameter:{}}}", key), value);
    }

    // Replace request body parameters
    if let Some(body) = request_body {
        if let Value::Object(ref mut map) = serde_json::from_str(&body).ok().expect("invalid json") {
        for pair in map {
                output = output.replace(&format!("{{request:{}}}", pair.0), &pair.1.as_str().unwrap());
        }
    }
}

    output
}
