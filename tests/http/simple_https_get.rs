use hyper::body::HttpBody as _;
use hyper::{Body, Client, Response};
use hyper_tls::HttpsConnector;
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn aux() -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();

    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = "https://graph.facebook.com/v10.0/?id=139608884711552&=".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(resp)
}

fn result() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = aux()?;

    Ok(())
}

pub fn main() {
    assert!(result().is_ok());
}
