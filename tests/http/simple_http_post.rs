use hyper::{Body, Client, Method, Request, Response, StatusCode};

use std::error::Error;

#[tokio::main]
async fn aux() -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let client = Client::new();

    // POST it...
    let resp = client.request(req).await?;

    assert_eq!(resp.status(), StatusCode::from_u16(200).unwrap());

    Ok(resp)
}

pub fn main() {
    assert!(aux().is_ok());
}
