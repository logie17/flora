use super::client;
use std::collections::HashMap;

pub struct Storage <'a>{
    clients: HashMap<&'a str, client::Client<'a>>
}

// TODO this will ultimately become a trait

impl <'a>Storage<'a> {
    pub fn new() -> Storage<'a> {
        let mut s = Storage{clients: HashMap::new()};
        s.clients.insert("foo", client::Client::new("foo", "12345", "http://www.foo.com"));
        return s;
    }

    pub fn GetClient(&self, client_id: &'a str) -> &client::Client<'a>{
        // This obviously nzeeds a lookup table of sorts
        if self.clients.contains_key(&client_id) {
            let found = self.clients.get(&client_id).unwrap();
            return found;
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn storage_lookup() {
        let s = super::Storage::new();
        assert_eq!(s.GetClient("abc").get_id(),("abc"));
    }
}


