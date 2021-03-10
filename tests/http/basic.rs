use hyper::{Body, Client, Method, Request};

#[tokio::main]
async fn aux() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let client = Client::new();

    // POST it...
    let resp = client.request(req).await?;
    println!("Response: {}", resp.status());

    Ok(())
}

pub fn main() {
    assert!(aux().is_ok());
}
