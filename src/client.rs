pub struct Client <'a> {
    id: &'a str,
    secret: &'a str,
    redirect_uri: &'a str,
}

impl <'a, 'b, 'c> Client<'a> {
    pub fn new(id: &'a str, secret: &'a str, redirect_uri: &'a str) -> Client<'a> {
        Client{id:id, secret: secret, redirect_uri: redirect_uri}
    }

    pub fn get_id(&self) -> &'a str {
        self.id
    }

    pub fn get_secret(&self) -> &'a str {
        self.secret
    }

    pub fn get_redirect_uri(&self) -> &'a str {
        self.redirect_uri
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn client_initialize_accessors() {
        let s = super::Client::new("abc123", "secret", "http://foo.com/cb");
        assert_eq!(s.get_id(),("abc123"));
        assert_eq!(s.get_secret(),("secret"));
        assert_eq!(s.get_redirect_uri(),("http://foo.com/cb"));
    }
}
