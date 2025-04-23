
    pub mod web_utils;
    pub mod template_engine;
    pub mod request_handler;
    
    // Re-export commonly used public functions/types for convenience
    pub use web_utils::{extract_route_parameter, extract_query_parameter};
    pub use template_engine::render_template;
    pub use request_handler::{handle_request, Endpoint};

