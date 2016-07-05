pub static ACCESS_DENIED: &'static str = "access_denied";
pub static SERVER_ERROR: &'static str = "server_error";
pub static UNAUTHORIZED_CLIENT: &'static str = "unauthorized_client";

pub struct Error {
    id: String,
    description: String,
}

impl <'a> Error {
    pub fn new(id: String, description: String) -> Error {
        Error{
            id: id,
            description: description,
        }
    }
}
