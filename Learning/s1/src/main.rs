use std::net::TcpListener;
fn main() {
    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");
}