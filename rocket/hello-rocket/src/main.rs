use rocket::fs::FileServer;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
#[macro_use] extern crate rocket;

// standard route with get from root or /
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// using dynamic paths
#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

// using multiple segments
#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// ignore segments, everything below a segment
#[get("/asegment/<_..>")]
fn everything() -> &'static str {
    "Hey, you're here."
}

//ignore segments in between segments
#[get("/foo/<_>/bar")]
fn foo_bar() -> &'static str {
    "Foo _____ bar!"
}

// using forwarding, a way to check types, before 404ing
#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!( "An unsigned integer called {}",id )
}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) -> String {
    format!( "An signed integer called {}",id )
}

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) -> String {
    format!( "A string called {}",id )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, files, foo_bar, everything, user, user_int, user_str])
    .mount("/public", FileServer::from("static/"))
}