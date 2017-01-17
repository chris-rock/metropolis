// extern crate libfritz;
//
// fn main() {
//     println!("Hello, Node!");
//     println!("Hello: {}", libfritz::fibonacci(20));
//     println!("Hello: {:?}", libfritz::get_content("http://fritz.box/login_sid.lua"));
//
//     println!("Hello: {:?}", libfritz::login());
// }


#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
