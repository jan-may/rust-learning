#[macro_use]
// disables warnings for unused variables
#[allow(unused_variables)]
extern crate rocket;

use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use rocket::fs::FileServer;
use rocket::request::{self, FromRequest, Request};
use rocket::{get, routes};
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::response;
use std::io::Cursor;

// custom request guard
struct AuthKeyGuard {
    key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthKeyGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Retrieve the value of the "key" parameter from the request.
        let key_param = req.query_value::<String>("key");

         if let Some(Ok(key)) = key_param {
            if is_valid_key(&key) {
                return request::Outcome::Success(AuthKeyGuard { key });
            }
        }

        request::Outcome::Failure((Status::Unauthorized, ()))
    }
}

fn is_valid_key(key: &String) -> bool {
    key == "secret" // Replace with your actual secret key.
}

#[get("/protected")]
async fn protected_route(auth_key: AuthKeyGuard) -> String {
    // This route will only be accessible if the authentication key is valid.
    format!("Welcome to the protected route with key: {}", auth_key.key)
}

#[get("/")]
fn index(jar: &CookieJar<'_>) -> &'static str {
    jar.add_private(Cookie::new("super_secret_key", "Sophie ist ein racer Marathoni ❤️"));
    "Hello, world!"
}

#[get("/cookie")]
fn index2(jar: &CookieJar<'_>) -> String {
    let cookie = jar.get_private("super_secret_key");
    format!("Hello, {}!", cookie.unwrap().value())}


#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}


// ignores all between -> could be more than one
#[get("/foo/<_..>/bar")]
fn ignore() -> &'static str {
    "Foo _____ bar!"
}


#[get("/user/<id>")]
fn user(id: usize) { println!("id: {}", id);}

#[get("/user/<id>", rank = 2)]
fn user_int(id: isize) { println!("id: {}", id); }

#[get("/user/<id>", rank = 3)]
fn user_str(id: &str) { println!("id: {}", id); }

struct User {
    name: String,
    id: usize
}

// check against content type header for payload methods
#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: String) { /* ... */ }

// check against accept header for non payload methods
#[get("/user/<id>", format = "json")]
fn get_user(id: usize) -> String {"User1".to_string() }



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, index2, hello, files, ignore])
        .mount("/public", FileServer::from("static/"))
        .mount("/", routes![user, user_int, user_str])
        .mount("/", routes![protected_route])
}
