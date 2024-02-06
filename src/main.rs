#[macro_use]
extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

/// Hello World!
///
/// Index page that returns "Hello World"
#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

/// Delay by Seconds
///
/// Wait x seconds until respondsing
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, delay])
}
