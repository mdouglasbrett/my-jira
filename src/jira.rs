use std::convert::Infallible;

use anyhow::Result;
// use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming as IncomingBody},
    Method, Request, Response, /*, StatusCode,*/
};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, Infallible>;

// Infallible is interesting. The Hyper example hides it away, but I feel
// like I should be explicit about it in the newtype. :shrug_emoji:
//let boxed_body = Full::from("Hi from my Jira server".to_owned()).boxed();
//let resp = Response::builder()
//    .status(StatusCode::OK)
//    .body(boxed_body)?;
//Ok(resp)

pub async fn jira(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/tickets/insert") => todo!(),
        // TODO: requires ?id={u64} to mutate item
        (&Method::POST, "/tickets/update") => todo!(),
        (&Method::GET, "/tickets/delete") => todo!(),
        // TODO: requires ?id={u64} for specific item, otherwise returns all
        (&Method::GET, "/tickets") => todo!(),
        _ => {
            // Return 404 not found response.
            todo!()
        }
    }
}
