use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...

    println!("new client incoming");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8099")?;
    println!("listening on :8099");
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}