pub struct AuthorizeRequest<'a> {
    redirect_uri: &'a str,
    state: &'a str,
    scope: &'a str,
    is_authorized: bool
}

impl <'a> AuthorizeRequest<'a> {
    pub fn new(redirect_uri: &'a str, state: &'a str, scope: &'a str) -> AuthorizeRequest<'a> {  
        AuthorizeRequest{redirect_uri: redirect_uri, state: state, scope:scope, is_authorized: false}
    }

}
