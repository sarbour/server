fn main() {
    let port = "127.0.0.1:8080".to_string();
    let server = Server::new(port);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}
