#![feature(async_await, await_macro)]
#[warn(unused_imports)]

// use futures::{Future, Stream};
use reqwest::r#async::{Client, Decoder};
use reqwest::header::HeaderMap;

fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("test-key1","test-value1".parse().unwrap());
    let client = reqwest::Client::new();
    let res = client
        .post("http://bn2zuuorhznlg73av35dcejv2m8cw1.burpcollaborator.net")
        //.get("http://xxxxx/xxx.xx")   //send GET request
        .header("user-agent","") //add a single http header
        .headers(headers)   //add multiple http headers
        .body("test body")  //add body
        .send();    //send this requsts
    println!("{:?}", res);
}

//  POST / HTTP/1.1
//  test-key2: test-value2
//  test-key: test-value
//  content-length: 9
//  user-agent: reqwest/0.9.18
//  accept: */*
//  accept-encoding: gzip
//  host: bn2zuuorhznlg73av35dcejv2m8cw1.burpcollaborator.net
//
//  test body
