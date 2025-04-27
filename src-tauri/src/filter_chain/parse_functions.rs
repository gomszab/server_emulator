use std::collections::HashMap;

pub(crate) fn split_path_and_query(path: &str) -> (&str, Option<&str>) {
    let mut parts = path.splitn(2, '?');
    let base = parts.next().unwrap_or("").trim_end_matches('?');
    let query = parts.next();
    (base, query)
}

pub(crate) fn parse_query(query: Option<&str>) -> HashMap<String, String> {
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
