//
// Marketplace Server
//

extern crate nickel;

use nickel::{Nickel, StaticFilesHandler};

fn main() {
    println!("Marketplace server is starting...");
    let mut server = Nickel::new();
    server.utilize(StaticFilesHandler::new("public/"));
    server.listen("127.0.0.1:8080").unwrap();
}
