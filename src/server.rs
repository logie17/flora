use super::authorize;
use super::storage;
use super::error;
extern crate uuid;
extern crate base64;
use uuid::Uuid;
use base64::{encode};
pub struct FloraServer<'a> {
    name: &'a str,
    storage: storage::Storage<'a>,
}

struct BasicAuth<'a> {
    client_id: &'a str,
    client_secret: &'a str,
}

const CODE: &'static str = "code";
const AUTH_EXPIRE: u32 = 250;
const ACCESS_EXPIRES: u32 = 3600;

impl <'a> BasicAuth<'a> {
    pub fn client_id(&self) -> &'a str {
        return self.client_id
    }

    pub fn client_secret(&self) -> &'a str {
        return self.client_secret
    }
}


impl <'a> FloraServer<'a> {
    /// Constructs a new `FloraServer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use flora::FloraServer;
    ///
    /// let server = flora::FloraServer::new("name of server");
    /// ```
    pub fn new(name: &'a str) -> FloraServer<'a> {
        FloraServer{name:name, storage: storage::Storage::new()}
    }

    /// Does initial this will return true of false.
    ///
    /// # Arguments
    ///
    /// * `response` - An AuthorizeResponse object.
    /// * `request`  - An AuthorizeRequest object.
    ///
    pub fn handle_authorize_request(&self, response: &'a mut authorize::AuthorizeResponse, request: &'a authorize::AuthorizeRequest) -> bool {
        // TODO decode redirect_uri
        let client_id: &str = request.client_id();
        let response_type: &str = request.response_type();
        if client_id != "" {
            let client = self.storage.get_client(client_id);
            let return_val = match client {
                Ok(client) => {
                    if client.get_redirect_uri() == "" {
                        response.set_error_state(error::UNAUTHORIZED_CLIENT.to_string(), "".to_string(), request.state().to_string());
                    }
                    let redirect_uri: &str = client.get_redirect_uri();
                    // TODO, add multiple redirect_uri support
                    response.redirect_uri(redirect_uri.to_string());
                    return match response_type {
                        CODE => {
                            response.code_type(CODE.to_string());
                            response.expiration(AUTH_EXPIRE);
                            true
                        },
                        _ => {
                            response.set_error_state(error::UNSUPPORTED_RESPONSE_TYPE.to_string(), "".to_string(), request.state().to_string());
                            false
                        }
                    };
                },
                Err(e) => {
                    response.set_error_state(error::SERVER_ERROR.to_string(), "".to_string(), request.state().to_string());
                    response.internal_error(e);
                    false
                }
            };
            return return_val;

        }

        return false;
    }

    /// Decorates the AuthorizeResponse with needed return data.
    ///
    /// # Arguments
    ///
    /// * `response` - An AuthorizeResponse object.
    /// * `request`  - An AuthorizeRequest object.
    ///
    pub fn finish_authorize_request(&mut self, response: &mut authorize::AuthorizeResponse, request: &authorize::AuthorizeRequest, is_authorized: bool) {
        if response.is_error() {
            return
        }

        if is_authorized {
            let v4 = Uuid::new_v4();
            let code = encode(&v4.urn().to_string()).to_string();
            let ret = authorize::AuthorizeData::new(code.clone());
            self.storage.save_authorize(ret);
            response.code(code);
        } else {
            response.set_error_state(error::ACCESS_DENIED.to_string(), "".to_string(), request.state().to_string());
        }
    }

    /// Does initial this will return true of false.
    ///
    /// # Arguments
    ///
    /// * `response` - An AuthorizeResponse object.
    /// * `request`  - An AuthorizeRequest object.
    ///
    pub fn handle_access_request(&self, response: &'a mut authorize::AuthorizeResponse, request: &'a authorize::AuthorizeRequest) -> bool {
        let grant_type: &'a str = request.grant_type();
        let return_val = match grant_type {
            CODE => {
                let basic_auth = self.get_client_auth(request);
                let client = self.storage.get_client(basic_auth.client_id());
                return match client {
                    Ok(client) => {
                        if client.get_redirect_uri() == "" {
                            response.set_error_state(error::UNAUTHORIZED_CLIENT.to_string(), "".to_string(), "".to_string());
                            return false
                        }

                        if client.get_secret() != basic_auth.client_secret() {
                            response.set_error_state(error::UNAUTHORIZED_CLIENT.to_string(), "".to_string(), "".to_string());
                            return false
                        }

                        // Create Access Request
                        let access_request = authorize::AccessRequest::new(
                            client.get_id(),
                            "CODE",
                            &request.code(),
                            &request.redirect_uri(),
                            true,
                            &ACCESS_EXPIRES,
                        );

                        let authorize = self.storage.get_authorize(&request.code());

                        // if authorize {
                        //     return false;
                        // }

                        // Load Authorize from storage


                        // Check expiration



                        true
                    },
                    Err(e) => {
                        response.set_error_state(error::SERVER_ERROR.to_string(), "".to_string(), request.state().to_string());
                        response.internal_error(e);
                        false
                    }
                };
            },
            _ => {
                response.set_error_state(error::UNSUPPORTED_GRANT_TYPE.to_string(), "".to_string(), "".to_string());
                false
            }
        };
        return return_val;
    }

    pub fn finish_access_request(&mut self, response: &'a mut authorize::AuthorizeResponse, request: &'a mut authorize::AuthorizeRequest, is_authorized: bool) {
        let redirect_uri: &str = request.redirect_uri();
        if is_authorized {
            let v4 = Uuid::new_v4();
            let access_token = encode(&v4.urn().to_string()).to_string();
            let refresh_token = encode(&v4.urn().to_string()).to_string();
            let ret = authorize::AccessData::new(
                "aabbcc".to_string(),
                access_token,
                refresh_token,
                ACCESS_EXPIRES,
            );
            self.storage.save_access(ret);
        }
    }

    // TODO - This sould probably be in util
    fn get_client_auth(&self, request: &'a authorize::AuthorizeRequest) -> BasicAuth<'a> {
        // Right now we will assume these are passed in via the request object
        // TODO: We will need a way to inspect HTTP headers
        BasicAuth{client_id:request.client_id(), client_secret: request.client_secret()}
    }



}
