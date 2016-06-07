pub struct AuthorizeRequest<'a> {
    client_id: &'a str,
    redirect_uri: &'a str,
    state: &'a str,
    scope: &'a str,
    is_authorized: bool
}

impl <'a> AuthorizeRequest<'a> {
    pub fn new(client_id: &'a str, redirect_uri: &'a str, state: &'a str, scope: &'a str) -> AuthorizeRequest<'a> {  
        AuthorizeRequest{
            client_id: client_id,
            redirect_uri: redirect_uri,
            state: state,
            scope:scope,
            is_authorized: false,
        }
    }

    pub fn client_id(&self) -> &'a str {
        return self.client_id
    }

}

pub struct AuthorizeResponse<'a> {
    redirect_uri: &'a str,
    code: &'a str,
    state: &'a str,
}

impl <'a>AuthorizeResponse<'a> {
    pub fn new() -> AuthorizeResponse<'a> {  
        AuthorizeResponse{redirect_uri:"", code:"", state: ""}
    }

    pub fn redirect_uri(&'a mut self, redirect_uri: &'a str) {
        self.redirect_uri = redirect_uri;
    }

    pub fn code(&'a mut self, code: &'a str) {
        self.code = code;
    }

    pub fn state(&'a mut self, state: &'a str) {
        self.state = state;
    }

}
