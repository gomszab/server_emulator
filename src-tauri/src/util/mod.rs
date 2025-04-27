
    pub mod request_handler;
    pub mod resolver;
    pub mod request_util;
    
    pub use request_handler::handle_request;
    pub use request_util::{Endpoint, RequestWrapper, create_request_wrapper};
    pub use resolver::{HtmlOrJson, get_value_as_string};

