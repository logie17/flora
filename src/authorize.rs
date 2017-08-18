extern crate time;

pub struct AuthorizeRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    response_type: &'a str,
    code: String,
    redirect_uri: &'a str,
    state: &'a str,
    scope: &'a str,
    grant_type: &'a str,
    is_authorized: bool
}

impl <'a> AuthorizeRequest<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str, redirect_uri: &'a str, state: &'a str, scope: &'a str, response_type: &'a str, grant_type: &'a str) -> AuthorizeRequest<'a> {
        AuthorizeRequest{
            client_id: client_id,
            client_secret: client_secret,
            redirect_uri: redirect_uri,
            state: state,
            scope:scope,
            response_type: response_type,
            is_authorized: false,
            grant_type: grant_type,
            code: "".to_string(),
        }
    }

    pub fn client_id(&self) -> &'a str {
        return self.client_id
    }

    pub fn client_secret(&self) -> &'a str {
        return self.client_secret
    }

    pub fn state(&self) -> &'a str {
        return self.state
    }

    pub fn response_type(&self) -> &'a str {
        return self.response_type
    }

    pub fn redirect_uri(&self) -> &'a str {
        return self.redirect_uri
    }

    pub fn grant_type(&self) -> &'a str {
        return self.grant_type
    }

    pub fn code(&self) -> String {
        return self.code;
    }
    pub fn set_code(&mut self, v: String) {
        self.code = v;
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

pub struct AccessRequest<'a> {
    client_id: &'a str,
    access_type: &'a str,
    code: &'a str,
    redirect_uri: &'a str,
    generate_refresh: bool,
    expiration: &'a u32,
}

impl <'a> AccessRequest<'a> {
    pub fn new(
        client_id: &'a str,
        access_type: &'a str,
        code: &'a str,
        redirect_uri: &'a str,
        generate_refresh: bool,
        expiration: &'a u32,
    ) -> AccessRequest<'a> {
        AccessRequest{
            client_id: client_id,
            access_type: access_type,
            code: code,
            redirect_uri: redirect_uri,
            generate_refresh: generate_refresh,
            expiration: expiration,
        }
    }
}


pub struct AccessData {
    redirect_uri: String,
    client_id: String,
    access_token: String,
    refresh_token: String,
    created_on: f64,
    expires_in: u32
}

impl <'a>AccessData {
    pub fn new(client_id: String, access_token: String, refresh_token: String, expires_in: u32) -> AccessData {
        AccessData{
            redirect_uri:"".to_string(),
            client_id:client_id,
            access_token: "".to_string(),
            refresh_token: "".to_string(),
            expires_in: expires_in,
            created_on: time::precise_time_s()
        }
    }

    pub fn redirect_uri(&'a mut self, redirect_uri: String) {
        self.redirect_uri = redirect_uri;
    }

    pub fn access_token(&'a mut self, access_token: String) {
        self.access_token = access_token;
    }

    pub fn get_access_token(&'a self) -> &String {
        return &self.access_token;
    }

    pub fn refresh_token(&'a mut self, refresh_token: String) {
        self.refresh_token = refresh_token;
    }
}
