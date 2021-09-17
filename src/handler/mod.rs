use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream}
};

const CRLF: &str = "\r\n";

pub async fn handle_request(mut stream: TcpStream){
    
    let mut buf:[u8; 4096] = [0; 4096];

    stream.read(&mut buf).await;

    if Self::matched("/index"){

    }else{

    }
}

fn matched(buf: [u8; 4096], route: &str) -> bool{
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    buf.starts_with(s, "GET")
}