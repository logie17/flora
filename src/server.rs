use super::authorize;
use super::storage;
use super::client;
use super::error;
extern crate uuid;
use uuid::Uuid;

pub struct FloraServer<'a> {
    name: &'a str,
    storage: storage::Storage<'a>,
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
    pub fn HandleAuthorizeRequest(&self, response: &'a mut authorize::AuthorizeResponse, request: &'a authorize::AuthorizeRequest) -> bool {
        // TODO decode redirect_uri
        let client_id: &'a str = request.client_id();
        if client_id != "" {
            let client = self.storage.GetClient(client_id);
            let return_val = match client {
                Ok(client) => {
                    if client.get_redirect_uri() == "" {
                        response.set_error_state(error::UNAUTHORIZED_CLIENT.to_string(), "".to_string(), request.state().to_string());
                    }
                    let redirect_uri: &'a str = client.get_redirect_uri();
                    response.redirect_uri(redirect_uri.to_string());
                    true
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
    pub fn FinishAuthorizeRequest(&mut self, response: &mut authorize::AuthorizeResponse, request: &authorize::AuthorizeRequest, is_authorized: bool) {
        if response.is_error() {
            return
        }

        if is_authorized {
            let v4 = Uuid::new_v4();
            let code = v4.urn().to_string();
            let ret = authorize::AuthorizeData::new(code.clone());
            self.storage.save_authorize(ret);
            response.code(code);
        } else {
            response.set_error_state(error::ACCESS_DENIED.to_string(), "".to_string(), request.state().to_string());
        }
    }

}
