use log::{info};

pub struct Server {
    address: String,
    port: i16
}

impl Server {
    pub fn new(address: String, port: i16) -> Self {
        Server{address, port}
    }

    pub fn start(&self) -> () {
        info!("Starting server");
    }
}
