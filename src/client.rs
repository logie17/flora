pub stuct Client <'a> {
    id: &'a str,
    secret: &'a str,
    redirect_uri: &'a str,
}

impl <'a> Client<'a> {
    pub fn new(id: &'a str) -> Client<'a> {
        Client{id:id, secret: secret, redirect_uri: redirect_uri}
    }
}
