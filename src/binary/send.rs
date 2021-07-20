use crate::binary::cancel::cancel;
use crate::binary::struct_trait::{send::Send, session::Session};
use hyper::client::ResponseFuture;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::boxed::Box;
use std::error::Error;
use std::io::Write;
use std::marker;
use std::net::TcpStream;
use std::panic;

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

/// Send a value of type `T` over http. Returns the
/// continuation of the session `S`. May fail.
pub fn send_http<T, S>(
    x: T,
    s: Send<T, S>,
    http: bool,
    method: Method,
    uri: &str,
    header: Vec<(&str, &str)>,
    body: &'static str,
) -> Result<(S, ResponseFuture), Box<dyn Error>>
where
    T: marker::Send,
    S: Session,
{
    let (here, there) = S::new();

    let respfut = match http {
        true => {
            let mut temp = Request::builder().method(method).uri(uri);

            for elt in header {
                temp = temp.header(elt.0, elt.1);
            }

            let req = temp.body(Body::from(body))?;

            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, Body>(https);

            client.request(req)
        }
        false => {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, Body>(https);

            client.request(Request::default())
        }
    };

    ////////////////

    match s.channel.send((x, there)) {
        Ok(_) => Ok((here, respfut)),
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
        Err(e) => {
            cancel(s);
            panic!("{}", e.to_string())
        }
    }
}
