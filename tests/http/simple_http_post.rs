// use hyper::body::HttpBody as _;
use hyper::{Body, Client, Method, Request, Response, StatusCode};
// use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn aux() -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
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

fn result() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = aux()?;

    Ok(())
}

pub fn main() {
    assert!(result().is_ok());
}
