#![feature(async_await, await_macro)]
#[warn(unused_imports)]

use reqwest::r#async::{Client, Decoder};
use std::collections::HashMap;
use reqwest::header::HeaderMap;
use std::env;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

#[macro_export]
macro_rules! Debuger {
    ($debug:expr, $val:expr) => {{
        match $val {
            tmp => {
                if $debug {
                    eprintln!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!(tmp_val), &tmp);
                }
                tmp
            }
        }
    }};
}

fn main() {
    let debug: bool = true;
    let hash_args: HashMap<String, String> = arg_format();

    Debuger!(debug, &hash_args);
    if Debuger!(debug, hash_args.get("-u") != None) {
        println!("{:?}", hash_args.get("-u"));
    }
    else {
        panic!("-u arg is uri, this arg must not None");
    }
    if Debuger!(debug, hash_args.get("-t") != None) {
        println!("{:?}", hash_args.get("-t"));
    }
    else {
        panic!("-t arg is type (client/server), this arg must noe None");
    }
    for i in hash_args {
        println!("{:?}", i);
    }
    // http_server("127.0.0.1:8099".to_string());
}

fn arg_format() -> HashMap<String, String> {
    let args :Vec<String> = env::args().collect();
    let debug: bool = true;
    let mut key;
    let mut value;
    let mut hash_args: HashMap<String, String> = HashMap::new();
    Debuger!(debug, &args);
    for n in 1..args.len() {
        Debuger!(debug, n);
        if n % 2 == 0 {
            key = args[n - 1].to_string();
            value = args[n].to_string();
            Debuger!(debug, ("key:", &key, "value", &value));
            hash_args.insert(key, value);
        }
    }
    return hash_args;
}

fn http_client(uri: String) {
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

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello, World!"))
}

fn http_server(uri: String) {
    let server;

    let hello = || {
        return service_fn_ok(hello_world);
    };

    match uri.parse::<std::net::SocketAddr>() {
        Ok(addr) => {
            server = Server::bind(&addr)
            .serve(hello)
            .map_err(|e| eprintln!("server error: {}", e));
        },
        Err(error) => {
            panic!("uri parse error:{:?}", error);
        },
    };

    hyper::rt::run(server);
}

// fn format_http_server_uri(uri: String) -> ([String;4], String) {
//     let ip;
//     let port;
//     let addr;
//     let mut tmp_vec: Vec<&str>;
//     {
//         tmp_vec = uri.split(":").collect::<Vec<&str>>();
//         ip = tmp_vec[0].to_string();
//         port = tmp_vec[1].to_string();
//     }

//     tmp_vec = ip.split(".").collect::<Vec<&str>>();

//     addr = (
//         Ipv4Addr::new(
//             tmp_vec[0].parse::<u8>(),
//             tmp_vec[1].parse::<u8>(), 
//             tmp_vec[2].parse::<u8>(),
//             tmp_vec[3].parse::<u8>()
//         ), port
//     );
//     return addr;
// }