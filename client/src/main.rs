use futures::{future, Sink, SinkExt, Stream, StreamExt};
use std::{error::Error, net::SocketAddr};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec, LinesCodecError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8734".to_string().parse::<SocketAddr>()?;
    let stdin = FramedRead::new(tokio::io::stdin(), LinesCodec::new());
    //let stdin = stdin.map(|i| i.map(|bytes| bytes.freeze()));
    let stdout = FramedWrite::new(tokio::io::stdout(), LinesCodec::new());

    connect(&addr, stdin, stdout).await?;
    Ok(())
}

async fn connect(
    addr: &SocketAddr,
    mut stdin: impl Stream<Item = Result<String, LinesCodecError>> + Unpin,
    mut stdout: impl Sink<String, Error = LinesCodecError> + Unpin,
) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr).await?;
    let (r, w) = stream.split();

    let mut sink = FramedWrite::new(w, LinesCodec::new());
    let mut stream = FramedRead::new(r, LinesCodec::new())
        .filter_map(|i| match i {
            Ok(i) => future::ready(Some(i)),
            Err(e) => {
                println!("Failed to read from socket; Error = {}", e);
                future::ready(None)
            }
        })
        .map(Ok);

    match future::join(sink.send_all(&mut stdin), stdout.send_all(&mut stream)).await {
        (Err(e), _) | (_, Err(e)) => Err(e.into()),
        _ => Ok(()),
    }
}
