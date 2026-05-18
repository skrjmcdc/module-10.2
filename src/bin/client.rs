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

    loop {
        match stdin.next_line().await {
            Ok(Some(line)) => {
                println!("You sent: {}", line);
                ws_stream.send(Message::text(line)).await?;
            }
            Ok(None) => break Ok(()),
            Err(err) => break Err(err.into()),
        }
    }
}
