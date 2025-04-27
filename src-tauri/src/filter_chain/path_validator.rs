use super::{
    chain::{RequestFilter, ValidationContext},
    parse_functions::split_path_and_query,
};

pub(crate) struct PathValidator;

impl RequestFilter for PathValidator {
    fn do_filter(&self, ctx: &mut ValidationContext) -> Option<()> {
        let request_url = &ctx.request.url().to_string();
        let (endpoint_base, _) = split_path_and_query(&ctx.endpoint.path);
        let (request_base, _) = split_path_and_query(request_url);

        let endpoint_segments: Vec<&str> = endpoint_base.trim_matches('/').split('/').collect();
        let request_segments: Vec<&str> = request_base.trim_matches('/').split('/').collect();

        if endpoint_segments.len() != request_segments.len() {
            return None;
        }

        for (e_seg, r_seg) in endpoint_segments.iter().zip(request_segments.iter()) {
            if e_seg.starts_with(':') {
                ctx.path_params
                    .insert(e_seg[1..].to_string(), r_seg.to_string());
            } else if e_seg != r_seg {
                return None;
            }
        }
        Some(())
    }
}
