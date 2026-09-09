use crate::server::Server;

pub mod server;
pub mod router;
pub mod handler;

fn main() {
let server = Server::new("127.0.0.1:12345");
    server.run();
}
