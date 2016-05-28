pub struct FloraServer<'a> {
    name: &'a str,
}

impl <'a> FloraServer<'a> {
    pub fn new(name: &'a str) -> FloraServer<'a> {
        FloraServer{name:name}
    }

  //  pub fn HandleAuthorizeRequest() -> flora::AuthorizeRequest {
  //      AuthorizeRequest{}
  //  }
}
