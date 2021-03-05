use crate::binary::struct_trait::{Send, Session};
use hyper::body::HttpBody as _;
use hyper::Client;
use std::boxed::Box;
use std::error::Error;
use std::io::Write;
use std::marker;
use std::net::TcpStream;
use std::panic;
use tokio::io::{stdout, AsyncWriteExt as _};

type TcpData = [u8; 128];

/// Send a value of type `T`. Always succeeds. Returns the
/// continuation of the session `S`.
pub fn send<T, S>(x: T, s: Send<T, S>) -> S
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    s.channel.send((x, there)).unwrap_or(());
    here
}

/// Send a value of type `T` over tcp. Returns the
/// continuation of the session `S`. May fail.
pub fn send_tcp<T, S>(
    x: T, // Need to force x and data to be of the same type but for choice/offer
    data: &TcpData,
    s: Send<(T, TcpData), S>,
    mut stream: TcpStream,
    tcp: bool,
) -> Result<(S, TcpStream), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    match s.channel.send(((x, *data), there)) {
        Ok(_) => {
            match tcp {
                true => {
                    // stream.shutdown(Shutdown::Read)?; // Force stream to be write only. Needed?
                    stream.write_all(data)?;
                    Ok((here, stream))
                }
                false => Ok((here, stream)),
            }
        }
        Err(e) => panic!("{}", e.to_string()),
    }
}

/// Send a value of type `T` over tcp. Returns the
/// continuation of the session `S`. May fail.
pub async fn send_http<T, S>(
    x: T,
    s: Send<T, S>,
    _http: bool,
) -> Result<S, Box<dyn Error + marker::Send + Sync>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();

    // Still inside `async fn main`...
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    ////////////////

    let uri = "http://httpbin.org/ip".parse()?;
    let mut resp = client.get(uri).await?;
    println!("Response: {}", resp.status());

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    match s.channel.send((x, there)) {
        Ok(_) => Ok(here),
        Err(e) => panic!("{}", e.to_string()),
    }
}

/// Send a value of type `T`. Always succeeds. Returns the
/// continuation of the session `S`.
pub fn send_canceled<T, S>(x: T, s: Send<T, S>) -> Result<S, Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();
    match s.channel.send((x, there)) {
        Ok(_) => Ok(here),
        Err(e) => panic!("{}", e.to_string()),
    }
}
