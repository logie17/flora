use super::authorize;

pub struct FloraServer<'a> {
    name: &'a str,
}

impl <'a> FloraServer<'a> {
    pub fn new(name: &'a str) -> FloraServer<'a> {
        FloraServer{name:name}
    }

    pub fn HandleAuthorizeRequest(redirect_uri: &'a str, state: &'a str, scope: &'a str) -> authorize::AuthorizeRequest<'a> {
        return authorize::AuthorizeRequest::new(redirect_uri, state, scope);
    }
}
