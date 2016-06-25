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
