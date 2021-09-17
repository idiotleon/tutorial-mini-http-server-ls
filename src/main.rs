use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop{
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move{
            let mut buf = [0; 1024];

            handler::handle_request(stream).await;

            loop{
                let n = match stream.read(&mut buf).await{
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // to write the data back
                if let Err(e) = stream.write_all(&buf[0..n]).await{
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}