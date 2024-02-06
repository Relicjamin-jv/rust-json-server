#[macro_use]
extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, delay])
}
