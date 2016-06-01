extern crate flora;

#[test]
fn authorization_code_flow_success() {
    let server = flora::server::FloraServer::new("foo");
    let auth_request = flora::authorize::AuthorizeRequest::new("abc123", "http://www.foo.com","","");
    let mut auth_response = flora::authorize::AuthorizeResponse::new();
    let is_authorized = server.HandleAuthorizeRequest(&mut auth_response, &auth_request);
    assert_eq!(is_authorized, true);
}

#[test]
fn authorization_code_flow_failure() {
    let server = flora::server::FloraServer::new("foo");
    let auth_request = flora::authorize::AuthorizeRequest::new("abc123", "","","");
    let mut auth_response = flora::authorize::AuthorizeResponse::new();
    let is_authorized = server.HandleAuthorizeRequest(&mut auth_response, &auth_request);
    assert_eq!(is_authorized, false);
}
