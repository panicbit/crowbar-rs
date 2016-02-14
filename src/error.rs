use std::io;
use serde_json as json;

wrapped_enum! {
    #[derive(Debug)]
    pub enum ParseError {
        /// std::io::Error
        Io(io::Error),
        /// serde_json::Error
        Json(json::Error)

    }
}
