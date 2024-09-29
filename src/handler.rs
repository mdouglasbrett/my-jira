use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::io::{BufReader, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Patch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    foo: String,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub data: Option<Data>, // TODO: headers? body
}

// TODO: what does a response look like?
pub struct Response;

// TODO: does _this_ have to be async?
fn handle_request(req: Request) -> Result<(), anyhow::Error> {
    let _ = match req.method {
        Method::Get => {
            println!("{:?}", req);
        }
        Method::Post => {
            println!("{:?}", req);
        }
        Method::Patch => {
            println!("{:?}", req);
        }
    };
    Ok(())
}

pub async fn handle_stream(stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut stream = BufReader::new(stream);
    let mut buf = String::new();
    stream.read_to_string(&mut buf).await?;
    let mut req_parts = buf.split_terminator("\r\n");
    // TODO: handle this better...
    let mut start_line = req_parts.next().expect("wut?").split_whitespace();

    let method = match start_line.next() {
        Some(s) => match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PATCH" => Method::Patch,
            _ => Method::Get,
        },
        None => {
            return Err(anyhow!("Could not get a method"));
        }
    };
    // TODO: could this be a slice instead?
    let uri = start_line.next().unwrap_or("/").to_string();

    // I don't actually care about the headers for this, I just want to get 
    // to the body...
    // TODO: better way?
    while let Some(header) = req_parts.next() {
        // Divider between head and body
        if header.is_empty() {
            break;
        }
    }
    let data = match req_parts.next() {
        Some(d) => Some(serde_json::from_str(d)?),
        None => None,
    };
    // TODO: write the outcome of this call back to the stream here,
    // or pass it back to main in the Result type?
    let _ = handle_request(Request { method, uri, data })?;
    // TODO: prolly come back to this after I have brought in the Ticket stuff
    stream.write_all("HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n".as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
