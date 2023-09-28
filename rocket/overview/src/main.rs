#[macro_use]
extern crate rocket;

use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use std::io;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world")] // <- route attribute
fn world() -> &'static str {
    // <- request handler
    "hello, world :)!"
}

#[get("/hi/<name>")]
fn hi(name: &str) -> String {
    format!("Hello, {}!", name)
}

// assync function with tokio
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt")).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![world])
        .mount("/", routes![hi])
        .mount("/", routes![delay])
        .mount("/", routes![blocking_task])
}
