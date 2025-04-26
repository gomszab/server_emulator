
    pub mod web_utils;
    pub mod template_engine;
    pub mod request_handler;
    pub mod types;
    pub mod request_util;
    pub mod html_responses;
    
    // Re-export commonly used public functions/types for convenience
    pub use web_utils::{extract_route_parameter, extract_query_parameter};
    pub use template_engine::render_template;
    pub use request_handler::handle_request;
    pub use request_util::{Endpoint, RequestWrapper, create_request_wrapper};
    pub use types::{HtmlOrJson, get_value_as_string};
    pub use html_responses::{add_item_html, find_by_id_html, find_by_queryparam, make_invalid_response_html, no_logic, not_found,return_dataset_html};

