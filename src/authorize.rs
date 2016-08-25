pub struct AuthorizeRequest<'a> {
    client_id: &'a str,
    response_type: &'a str,
    redirect_uri: &'a str,
    state: &'a str,
    scope: &'a str,
    grant_type: &'a str,
    is_authorized: bool
}

impl <'a> AuthorizeRequest<'a> {
    pub fn new(client_id: &'a str, redirect_uri: &'a str, state: &'a str, scope: &'a str, response_type: &'a str, grant_type: &'a str) -> AuthorizeRequest<'a> {  
        AuthorizeRequest{
            client_id: client_id,
            redirect_uri: redirect_uri,
            state: state,
            scope:scope,
            response_type: response_type,
            is_authorized: false,
            grant_type: grant_type,
        }
    }

    pub fn client_id(&self) -> &'a str {
        return self.client_id
    }

    pub fn state(&self) -> &'a str {
        return self.state
    }

    pub fn response_type(&self) -> &'a str {
        return self.response_type
    }

    pub fn grant_type(&self) -> &'a str {
        return self.grant_type
    }

}

pub struct AuthorizeResponse {
    redirect_uri: String,
    code: String,
    code_type: String,
    expiration: u32,
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
            expiration: 0,
            code_type: "".to_string(),
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

    pub fn code_type(&'a mut self, code_type: String) {
        self.code_type = code_type;
    }

    pub fn expiration(&'a mut self, expiration: u32) {
        self.expiration = expiration;
    }

    pub fn get_expiration(&'a self) -> u32 {
        return self.expiration;
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
