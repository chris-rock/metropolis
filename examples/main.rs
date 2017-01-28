#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml;

// use std::env;
use std::io::{self, Write};

use futures::Future;
use futures::stream::Stream;

use hyper::Client;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct SessionInfo {
    Sid: String,
    Challenge: String,
    BlockTime: i32,
}

fn main() {
    // <SID>0000000000000000</SID><Challenge>2bdbdb68</Challenge><BlockTime>0</BlockTime><Rights></Rights>
    let serialized = "<SessionInfo><Sid>0000000000000000</Sid><Challenge>2bdbdb68</Challenge><BlockTime>0</BlockTime><rights></rights></SessionInfo>";

    // Convert the XML string back to a Point.
    let deserialized: SessionInfo = serde_xml::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    let url = "http://fritz.box/login_sid.lua";

    let url = hyper::Url::parse(&url).unwrap();
    if url.scheme() != "http" {
        println!("This example only works with 'http' URLs.");
        return;
    }

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    // future
    let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());
        res
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {

        println!("\n\nDone.");
    });

    core.run(work).unwrap();
}
