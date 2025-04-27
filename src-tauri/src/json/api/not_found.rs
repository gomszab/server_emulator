use tiny_http::Response;

use super::util::create_json_response;

pub fn not_found_json() -> Response<std::io::Cursor<Vec<u8>>> {
    create_json_response(r#"{"message": "Unsupported action"}"#, 400)
}
