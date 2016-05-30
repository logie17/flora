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

    pub fn client_id(&self) -> &str {
        return self.client_id
    }

}

pub struct AuthorizeResponse {}

impl AuthorizeResponse {
    pub fn new() -> AuthorizeResponse {  
        AuthorizeResponse{}
    }

}
