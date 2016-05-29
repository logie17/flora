pub struct Storage{}

// TODO this will ultimately become a trait

impl Storage {
    pub fn new() -> Storage {
        Storage{}
    }

    // lifetime is probably wrong
    pub fn GetClient(&self, client_id: &'static str) -> &str{
        return client_id
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn storage_lookup() {
        let s = super::Storage::new();
        assert_eq!(s.GetClient("abc"),("abc"));
    }
}


