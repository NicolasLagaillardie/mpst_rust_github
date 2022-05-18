use hyper::{Body, Client, Response, StatusCode};
use hyper_tls::HttpsConnector;
use std::error::Error;

#[tokio::main]
async fn aux() -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let https = HttpsConnector::new();

    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = "https://graph.facebook.com/v10.0/?id=139608884711552&=".parse()?;

    // Await the response...
    // let mut resp = client.get(uri).await?;

    // And now...
    // while let Some(chunk) = resp.body_mut().data().await {
    //     stdout().write_all(&chunk?).await?;
    // }

    // Ok(resp)

    let resp = client.get(uri).await?;

    assert_eq!(resp.status(), StatusCode::from_u16(200).unwrap());

    Ok(resp)
}

pub fn main() {
    assert!(aux().is_ok());
}
