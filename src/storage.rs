pub struct Storage{}
use super::client;

// TODO this will ultimately become a trait

impl Storage {
    pub fn new() -> Storage {
        Storage{}
    }

    pub fn GetClient<'a>(&self, client_id: &'a str) -> client::Client<'a>{
        // This obviously needs a lookup table of sorts
        return client::Client::new(client_id, "12345", "http://www.foo.com");
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


