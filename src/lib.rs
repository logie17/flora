pub use self::server::FloraServer;
pub mod server;
pub mod authorize;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1,1)
    }
}



