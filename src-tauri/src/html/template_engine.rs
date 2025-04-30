use serde_json::Value;
use std::{collections::HashMap, sync::Mutex};

use crate::util::get_value_as_string;

enum DataSetOrSingleValue<'a> {
    Dataset(&'a Mutex<Vec<Value>>),
    SingleValue(&'a Value),
}

trait DataSetOrSingleValueTrait {
    fn to_dataset_single(&self) -> DataSetOrSingleValue;
}

impl DataSetOrSingleValueTrait for &Mutex<Vec<Value>> {
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
    data: Option<T>,
    route_params: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
    request_body: Option<Value>,
) -> String {
    let mut output = template.to_string();
    if let Some(datasetparam) = data {
        if output.contains("{memory:dataset}") {
            if let DataSetOrSingleValue::Dataset(dataset) = &datasetparam.to_dataset_single() {
                let mut dataset = dataset.lock().unwrap();
                let mappedrows: Vec<String> = dataset
                    .iter_mut()
                    .map(|element| {
                        let mut result = String::new();
                        if let Value::Object(ref mut map) = element {
                            result.push_str("<tr>");
                            for pair in map {
                                if let Some(raw_value) = pair.1.as_str() {
                                    result.push_str(format!("<td>{}</td>", raw_value).as_str());
                                } else {
                                    // Fallback for non-string values
                                    result.push_str(
                                        format!("<td>{}</td>", pair.1.to_string()).as_str(),
                                    );
                                }
                            }
                            result.push_str("</tr>");
                        }
                        result
                    })
                    .collect();
                output = output.replace("{memory:dataset}", &format!("{}", mappedrows.join("")));
            }
        }

        if output.contains("{memory:element}") {
            if let DataSetOrSingleValue::SingleValue(element) = &datasetparam.to_dataset_single() {
                let mut result = String::new();
                if let Value::Object(map) = element {
                    for pair in map {
                        if let Some(raw_value) = pair.1.as_str() {
                            result.push_str(format!("<h1>{}</h1>", raw_value).as_str());
                        } else {
                            // Fallback for non-string values
                            result.push_str(format!("<h1>{}</h1>", pair.1.to_string()).as_str());
                        }
                    }
                    output = output.replace("{memory:element}", &format!("{}", result));
                }
            }
        }
    }
    // Handle dataset replacement

    // Replace route parameters
    if let Some(routes) = route_params {
        for (key, value) in routes.into_iter() {
            output = output.replace(&format!("{{pathparam:{}}}", key), value.as_str());
        }
    }

    if let Some(queries) = query_params {
        for (key, value) in queries.into_iter() {
            output = output.replace(&format!("{{queryparameter:{}}}", key), value.as_str());
        }
    }
    // Replace query parameters

    // Replace request body parameters
    if let Some(body) = request_body {
        if let Value::Object(map) = &body {
            for pair in map {
                output = output.replace(
                    &format!("{{request:{}}}", pair.0),
                    get_value_as_string(pair.1).as_str(),
                );
            }
        }
    }

    output
}
