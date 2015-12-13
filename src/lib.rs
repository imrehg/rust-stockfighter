#[macro_use] extern crate hyper;

use std::io;
use std::io::Read;

use hyper::Client;
use hyper::header::Headers;

header! { (XRequestGuid, "X-Request-Guid") => [String] }

pub fn heartbeat() -> io::Result<()> {

    let mut client = Client::new();
    let mut res = client.get("https://api.stockfighter.io/ob/api/heartbeat")
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
    Ok(())
}

#[test]
fn it_works() {
}
