use super::authorize;
use super::storage;

pub struct FloraServer<'a> {
    name: &'a str,
    storage: storage::Storage,
}

impl <'a> FloraServer<'a> {
    pub fn new(name: &'a str) -> FloraServer<'a> {
        FloraServer{name:name, storage: storage::Storage::new()}
    }
//&self, client_id: &'a str, redirect_uri: &'a str, state: &'a str, scope: &'a str
    pub fn HandleAuthorizeRequest(&self, response: &'a authorize::AuthorizeResponse, request: &'a authorize::AuthorizeRequest) -> bool {
        // TODO decode redirect_uri

        if request.client_id() != "" {
            let client = self.storage.GetClient(request.client_id());
            if client.get_redirect_uri() == "" {
                //todo better error handlling
            }
        }
        
//        let ret = authorize::AuthorizeRequest::new(redirect_uri, state, scope);
        

        return true;
    }

}
