use super::client;
use super::authorize;
use std::collections::HashMap;
use std::error::Error;

pub struct Storage <'a>{
    clients: HashMap<&'a str, client::Client<'a>>,
    authorizations: HashMap<String, authorize::AuthorizeData>
}

// TODO this will ultimately become a trait

impl <'a>Storage<'a> {
    pub fn new() -> Storage<'a> {
        let mut s = Storage{clients: HashMap::new(), authorizations: HashMap::new()};
        s.clients.insert("abc123", client::Client::new("abc123", "12345", "http://www.foo.com"));
        return s;
    }

    pub fn GetClient(&self, client_id: &'a str) -> Result<&client::Client<'a>, String> {
        let o = self.clients.get(&client_id);
        let found = match o {
            Some(o) => Ok(o),
            None => Err("No client id was found!".to_string()),
        };
        return found;
    }

    pub fn save_client(&mut self, client: client::Client) {
        
    }

    pub fn save_authorize(&mut self, authorize_data: authorize::AuthorizeData) {
        self.authorizations.insert(authorize_data.get_code().clone(), authorize_data);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn storage_lookup_and_save() {
        let s = super::Storage::new();
        let client = match s.GetClient("abc123") {
            Ok(client) => client,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(client.get_id(),"abc123");
    }

}


