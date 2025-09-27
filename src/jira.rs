use anyhow::Result;
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming as IncomingBody},
    Request, Response, StatusCode,
};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub async fn jira(_req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(
            Full::new("Hi from my Jira server".into())
                .map_err(|never| match never {})
                .boxed(),
        )
        .unwrap())
}
