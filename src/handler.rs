use anyhow::anyhow;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Patch,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    // TODO: headers? body
}

// TODO: what does a response look like?
// How is it written back to the stream?
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

pub async fn handle_stream(mut stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut buf = String::new();
    let _ = stream.read_to_string(&mut buf).await?;

    let mut req_parts = buf.split_whitespace();

    let method = match req_parts.next() {
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
    let path = req_parts.next().unwrap_or("/").to_string();
    // TODO: write this back to the stream?
    let _ = handle_request(Request { method, path })?;
    Ok(())
}
