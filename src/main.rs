// explain this code below
use std::sync::{Arc, Mutex};

struct RoundRobin {
    servers: Vec<String>,
    index: usize,
}

impl RoundRobin {
    fn new(servers: Vec<String>) -> Self {
        RoundRobin { servers, index: 0 }
    }

    fn next_server(&mut self) -> &str {
        if self.index >= self.servers.len() {
            self.index = 0;
        }
        let server = &self.servers[self.index];
        self.index += 1;
        server
    }
}

fn main() {
    let servers = vec![
        "http://server1.com".to_string(),
        "http://server2.com".to_string(),
        "http://server3.com".to_string(),
    ];

    let round_robin: Arc<Mutex<RoundRobin>> = Arc::new(Mutex::new(RoundRobin::new(servers)));

    for i in 0..10 {
        let mut rr = round_robin.lock().unwrap();
        let server = rr.next_server();
        println!("Request {}: Sending to {}", i + 1, server);
    }
}

