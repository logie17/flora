pub use self::server::FloraServer;
extern crate uuid;

pub mod server;
pub mod authorize;
pub mod storage;
pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1,1)
    }
}



