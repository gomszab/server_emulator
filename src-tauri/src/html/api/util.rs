use tiny_http::{Header, Response};

pub(crate) const HTML_RESPONSE: &str = "Content-Type: text/html; charset=utf-8";

pub(crate) fn make_invalid_response_html(message: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(message)
        .with_header::<Header>(HTML_RESPONSE.parse().unwrap())
        .with_status_code(400)
}
