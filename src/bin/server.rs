use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // Dengarkan pesan masuk dari klien ini.
            Some(msg_result) = ws_stream.next() => {
                let msg = msg_result?;
                println!("Received message from {addr:?}: {msg:?}");
                
                let _ = bcast_tx.send(format!("Message from {addr:?}: {msg:?}"));
            }

            // Dengarkan pesan broadcast dari klien lain, lalu kirim ke klien ini.
            Ok(msg) = bcast_rx.recv() => {
                println!("Broadcasting message to {addr:?}: {msg:?}");
                
                // Catatan: Pastikan Message::text atau Message::Text sesuai dengan versi tokio-websockets mu.
                ws_stream.send(Message::text(msg)).await?; 
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}