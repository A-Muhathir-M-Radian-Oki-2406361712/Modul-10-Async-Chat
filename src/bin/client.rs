use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    // TODO: For a hint, see the description of the task below.
loop {
        tokio::select! {
            // Cabang 1: Menunggu input dari keyboard (stdin)
            line_result = stdin.next_line() => {
                match line_result {
                    Ok(Some(line)) => {
                        let line = line.trim().to_string();
                        if !line.is_empty() {
                            // Kirim ke server
                            ws_stream.send(Message::text(line)).await?;
                        }
                    }
                    Ok(None) => {
                        println!("EOF reached. Exiting.");
                        break; // Keluar dari loop jika EOF (Ctrl+D)
                    }
                    Err(e) => {
                        eprintln!("Error reading from stdin: {e}");
                        break;
                    }
                }
            }

            // Cabang 2: Menunggu pesan broadcast dari server (websocket)
            msg_result = ws_stream.next() => {
                match msg_result {
                    Some(Ok(msg)) => {
                        // Tampilkan pesan broadcast yang diterima
                        println!("Received broadcast: {msg:?}");
                    }
                    Some(Err(e)) => {
                        eprintln!("Error receiving from WebSocket: {e}");
                        break;
                    }
                    None => {
                        println!("Server closed the connection.");
                        break; // Keluar dari loop jika koneksi terputus
                    }
                }
            }
        }
    }

    Ok(())
}