use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::transport::http::recv::recv_http;
use mpstthree::transport::http::send::send_http;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

type ClientMPST<N> = Send<N, Recv<N, End>>;
type ServerMPST<N> = <ClientMPST<N> as Session>::Dual;

fn client(s: ClientMPST<i32>) -> Result<(), Box<dyn Error>> {
    match fs::read_to_string("imgur.env") {
        Ok(contents) => {
            let lines: Vec<&str> = contents.split('\n').collect();
            let hasher = RandomState::new();
            let mut ids: HashMap<&str, &str> = HashMap::with_hasher(hasher);
            for line in lines {
                let temp: Vec<&str> = line.split('=').collect();
                ids.insert(temp[0], temp[1]);
            }

            let mut header: Vec<(&str, &str)> = Vec::new();

            let authorization = format!("{} {}", ids["TOKEN_TYPE"], ids["ACCESS_TOKEN"]);

            header.push(("content-type", ids["CONTENT_TYPE"]));
            header.push(("Authorization", &authorization[..]));
            header.push(("User-Agent", ids["USER_AGENT"]));
            header.push(("Accept", ids["ACCEPT"]));
            header.push(("Connection", ids["CONNECTION"]));

            /////////////

            let (s, req) = send_http(0, s, true, Method::GET, ids["CREDITS_URL"], header, "")?;
            let (_result, s, resp) = recv_http(s, true, req)?;

            assert_eq!(resp.status(), StatusCode::from_u16(200).unwrap());

            close(s)
        }
        Err(_) => {
            let (s, req) = send_http(0, s, false, Method::GET, "", vec![("", "")], "")?;
            let (_result, s, _resp) = recv_http(s, false, req)?;

            close(s)
        }
    }
}

pub fn main() {
    assert!({
        let s: ServerMPST<i32> = fork(client);
        //-------
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        //-------
        let (_result, s, _resp) = recv_http(s, false, client.request(Request::default()))?;
        let (s, _req) = send_http(0, s, false, Method::GET, "", vec![("", "")], "")?;
        close(s)
    }
    .is_ok());
}
