#[macro_use]
extern crate serde_derive;

extern crate serde_xml;

extern crate hyper;

use ::hyper::{Client};
use std::io::Read;

#[no_mangle]
pub extern fn fibonacci(x: i32) -> i32 {
  if x <= 2 {
    return 1;
  } else {
    return fibonacci(x - 1) + fibonacci(x - 2);
  }
}

#[no_mangle]
pub extern fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = client.get(url).send()?;
    let mut buf = String::new();
    response.read_to_string(&mut buf)?;
    Ok(buf)
}

use std::fmt::Debug;

use serde_xml::from_str;
use serde_xml::value::{Element, from_value};
use serde_xml::Error;

use serde::de;
use serde::ser;

#[derive(Serialize, Deserialize, Debug)]
struct SessionInfo {
    sid: String,
    challenge: String,
    blocktime: i32
}


pub fn login() -> String {
    let server = "http://fritz.box/login_sid.lua";
    let username = "username";
    let password = "password";

    let res = get_content(server);
    if res.is_ok(){
        println!("Call was successful");
    }

    let content = res.unwrap();
    // let xml_value: Element = from_str(content.as_str()).unwrap();

    let sessioninfo: SessionInfo = serde_xml::from_str(content.as_str()).unwrap();
    println!("data: {:?}", sessioninfo);
    return content;
}
