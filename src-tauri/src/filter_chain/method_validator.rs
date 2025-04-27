use super::chain::{self, RequestFilter};

pub struct MethodValidator;

impl RequestFilter for MethodValidator{
    fn do_filter(&self, ctx: &mut chain::ValidationContext) -> Option<()> {
        if ctx.endpoint.method == ctx.request.method().to_string() {
            Some(())
        } else {
            None
        }
    }
}