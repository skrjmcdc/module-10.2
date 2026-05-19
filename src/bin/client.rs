use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("> {}", text);
                        }
                    },
                    Some(Err(err)) => break Err(err),
                    None => break Ok(()),
                }
            }
            res = stdin.next_line() => {
                match res {
                    Ok(Some(line)) => {
                        ws_stream.send(Message::text(line)).await?;
                        println!("Message sent!");
                    }
                    Ok(None) => break Ok(()),
                    Err(err) => break Err(err.into()),
                }
            }
        }
    }
}
