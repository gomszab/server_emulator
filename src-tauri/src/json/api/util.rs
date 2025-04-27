use tiny_http::{Header, Response};

pub(crate) fn create_json_response(
    message: &str,
    statuscode: i32,
) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(message)
        .with_header::<Header>("Content-Type: application/json".parse().unwrap())
        .with_status_code(statuscode)
}
