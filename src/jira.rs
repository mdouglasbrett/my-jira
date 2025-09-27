use std::convert::Infallible;

use anyhow::Result;
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming as IncomingBody},
    Request, Response, StatusCode,
};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

pub async fn jira(_req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    // Infallible is interesting. The Hyper example hides it away, but I feel
    // like I should be explicit about it in the newtype. :shrug_emoji:
    let boxed_body = Full::from("Hi from my Jira server".to_owned()).boxed();
    let resp = Response::builder()
        .status(StatusCode::OK)
        .body(boxed_body)?;
    Ok(resp)
}
