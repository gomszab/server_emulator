use tiny_http::Response;

use super::util::make_invalid_response_html;

pub fn not_found_html() -> Response<std::io::Cursor<Vec<u8>>> {
    return make_invalid_response_html("<h1>Page not found</h1>");
}
