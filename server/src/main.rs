use tokio::net::TcpListener;
use tokio::sync::Mutex;

use rust_chat_server::{process, Shared};
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(Shared::new()));
    let addr = "127.0.0.1:8734".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", &addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            if let Err(e) = process(state, stream, addr).await {
                println!("An error occured; Error = {:?}", e);
            }
        });
    }
}
