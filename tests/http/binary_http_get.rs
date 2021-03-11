use hyper::{Method, Request, StatusCode};
use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork;
use mpstthree::binary::recv::recv_http;
use mpstthree::binary::send::send_http;
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

type Client<N> = Send<N, Recv<N, End>>;
type Server<N> = <Client<N> as Session>::Dual;

fn client(s: Client<i32>) -> Result<(), Box<dyn Error>> {
    /////////////
    // Get variables from file

    let contents = fs::read_to_string("imgur.env")?;
    let lines: Vec<&str> = contents.split("\n").collect();
    let hasher = RandomState::new();
    let mut ids: HashMap<&str, &str> = HashMap::with_hasher(hasher);
    for line in lines {
        let temp: Vec<&str> = line.split("=").collect();
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

    close(s)?;
    Ok(())
}

pub fn main() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        let s: Server<i32> = fork(client);
        let (_result, s, _resp) = recv_http(s, false, Request::default())?;
        let (s, _req) = send_http(0, s, false, Method::GET, "", vec![("", "")], "")?;
        close(s)?;
        Ok(())
    }()
    .is_ok());
}
