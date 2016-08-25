extern crate flora;
extern crate regex;
use regex::Regex;

#[test]
fn authorization_code_flow_success() {
    let mut server = flora::server::FloraServer::new("foo");
    let auth_request = flora::authorize::AuthorizeRequest::new("abc123", "http://www.foo.com","","", "code", "");
    let mut auth_response = flora::authorize::AuthorizeResponse::new();

    let is_authorized = server.HandleAuthorizeRequest(&mut auth_response, &auth_request);
    assert_eq!(is_authorized, true);
    assert_eq!(auth_response.get_expiration(), 250);

    server.FinishAuthorizeRequest(&mut auth_response, &auth_request, is_authorized);
    let re = Regex::new(r"^.*$").unwrap();
    assert!(re.is_match(auth_response.get_code()));
}

#[test]
fn authorization_code_flow_failure_invalid_grant_type() {
    let mut server = flora::server::FloraServer::new("foo");
    let auth_request = flora::authorize::AuthorizeRequest::new("abc123", "http://www.foo.com","","", "code", "unknown");
    let mut auth_response = flora::authorize::AuthorizeResponse::new();

    let is_authorized = server.HandleAuthorizeRequest(&mut auth_response, &auth_request);
    assert_eq!(is_authorized, true);
    assert_eq!(auth_response.get_expiration(), 250);

    server.FinishAuthorizeRequest(&mut auth_response, &auth_request, is_authorized);
    let re = Regex::new(r"^.*$").unwrap();
    assert!(re.is_match(auth_response.get_code()));

    let has_access = server.HandleAccessRequest(&mut auth_response, &auth_request);
    assert_eq!(has_access, false);
}

#[test]
fn authorization_code_flow_failure() {
    let server = flora::server::FloraServer::new("foo");
    let auth_request = flora::authorize::AuthorizeRequest::new("not-found", "","","", "foo", "");
    let mut auth_response = flora::authorize::AuthorizeResponse::new();
    let is_authorized = server.HandleAuthorizeRequest(&mut auth_response, &auth_request);
    assert_eq!(is_authorized, false);
}
