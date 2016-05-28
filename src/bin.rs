extern crate flora;

pub fn main() {
  let server = flora::server::FloraServer::new("foo");
  let r = flora::authorize::AuthorizeRequest{};
}
