use urlencoding::decode;

use super::{chain::{RequestFilter, ValidationContext}, parse_functions::{parse_query, split_path_and_query}};

pub(crate) struct QueryValidator;

impl RequestFilter for QueryValidator {
    fn do_filter(&self, ctx: &mut ValidationContext) -> Option<()> {
        let request_url = &ctx.request.url().to_string();
        let (_, endpoint_query) = split_path_and_query(&ctx.endpoint.path);
        let (_, request_query) = split_path_and_query(request_url);

        let endpoint_q = parse_query(endpoint_query);
        let request_q = parse_query(request_query);

        if endpoint_q.is_empty() {
            return if request_q.is_empty() { Some(()) } else { None };
        }

        if endpoint_q.len() != request_q.len() {
            return None;
        }

        for (key, e_val) in &endpoint_q {
            if let Some(r_val) = request_q.get(key) {
                if e_val.starts_with('<') && e_val.ends_with('>') {
                    let placeholder = e_val[1..e_val.len()-1].to_string();
                    let decoded = decode(r_val).unwrap_or(std::borrow::Cow::Owned(r_val.to_owned()));
                    ctx.query_params.insert(placeholder, decoded.into_owned());
                } else if e_val != r_val {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(())
    }
}