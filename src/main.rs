mod reverse_proxy;
use crate::reverse_proxy::server::Server;

fn main() {
    let server = Server::new("0.0.0.0".to_string(), 4200);

    server.start();
}
