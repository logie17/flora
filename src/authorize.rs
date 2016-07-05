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

    pub fn state(&self) -> &'a str {
        return self.state
    }

}

pub struct AuthorizeResponse {
    redirect_uri: String,
    code: String,
    state: String,
    error_id: String,
    error_description: String,
    is_error: bool,
    internal_error: String,
}

impl <'a>AuthorizeResponse {
    pub fn new() -> AuthorizeResponse {  
        AuthorizeResponse{
            redirect_uri:"".to_string(),
            code:"".to_string(),
            state: "".to_string(),
            error_id: "".to_string(),
            error_description: "".to_string(),
            is_error: false,
            internal_error: "".to_string(),
        }
    }

    pub fn redirect_uri(&'a mut self, redirect_uri: String) {
        self.redirect_uri = redirect_uri;
    }

    pub fn code(&'a mut self, code: String) {
        self.code = code;
    }

    pub fn get_code(&'a self) -> &String {
        return &self.code;
    }

    pub fn state(&'a mut self, state: String) {
        self.state = state;
    }

    pub fn is_error(&'a self) -> bool {
        return self.is_error;
    }

    pub fn internal_error(&'a mut self, error: String) {
        self.internal_error = error;
    }

    pub fn set_error_state(&'a mut self, error_id: String, description: String, state: String) {
        self.is_error = true;
        self.error_id = error_id;
        self.error_description = description;
        if self.state == "" {
            self.state = state
        }
    }
}

pub struct AuthorizeData {
    redirect_uri: String,
    code: String,
    state: String,
//    user_data: T,
}

impl <'a>AuthorizeData {
    pub fn new(code: String) -> AuthorizeData {  
        AuthorizeData{redirect_uri:"".to_string(), code:code, state: "".to_string()}
    }

    pub fn redirect_uri(&'a mut self, redirect_uri: String) {
        self.redirect_uri = redirect_uri;
    }

    pub fn code(&'a mut self, code: String) {
        self.code = code;
    }

    pub fn get_code(&'a self) -> &String {
        return &self.code;
    }

    pub fn state(&'a mut self, state: String) {
        self.state = state;
    }


}
