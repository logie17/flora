use super::authorize;
use super::storage;
use super::client;

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

    pub fn HandleAuthorizeRequest(&self, response: &'a mut authorize::AuthorizeResponse<'a>, request: &'a authorize::AuthorizeRequest) -> bool {
        // TODO decode redirect_uri
        let client_id: &'a str = request.client_id();
        if client_id != "" {
            let client = self.storage.GetClient(client_id);
            let return_val = match client {
                Ok(client) => {
                    if client.get_redirect_uri() == "" {
                        //todo better error handlling
                    }
                    let redirect_uri: &'a str = client.get_redirect_uri();
                    response.redirect_uri(redirect_uri);
                    true
                },
                Err(e) => {
                    print!("Here we throw a 5xx/4xx! {}", e);
                    false
                }
            };
            return return_val;
            
        }

        return false;
    }

    pub fn FinishAuthorizeRequest(&self, response: &'a mut authorize::AuthorizeResponse<'a>, request: &'a authorize::AuthorizeRequest) {

    }

}
