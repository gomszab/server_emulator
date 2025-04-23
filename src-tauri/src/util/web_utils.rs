use regex::Regex;

pub fn extract_route_parameter(path_pattern: &str, url: &str, param_name: &str) -> Option<String> {
    let param_pattern = format!(":{}", param_name);
    let regex_pattern = path_pattern
        .replace(&param_pattern, r"(?P<param>[^/]+)")
        .replace("/", r"\/");
    let full_regex = format!("^{}$", regex_pattern);
    let re = Regex::new(&full_regex).ok()?;

    re.captures(url)?
        .name("param")
        .map(|m| m.as_str().to_string())
}

pub fn extract_query_parameter(url: &str, param_name: &str) -> Option<String> {
    url.split('?').nth(1)? 
        .split('&')
        .find(|s| s.starts_with(&format!("{}=", param_name)))?
        .split('=')
        .nth(1)
        .map(|s| s.to_string()) 
}