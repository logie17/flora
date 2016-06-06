use super::client;
use std::collections::HashMap;
use std::error::Error;

pub struct Storage <'a>{
    clients: HashMap<&'a str, client::Client<'a>>
}

// TODO this will ultimately become a trait

impl <'a>Storage<'a> {
    pub fn new() -> Storage<'a> {
        let mut s = Storage{clients: HashMap::new()};
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn storage_lookup() {
        let s = super::Storage::new();
        let client = match s.GetClient("abc123") {
            Ok(client) => client,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(client.get_id(),"abc123");
    }
}


