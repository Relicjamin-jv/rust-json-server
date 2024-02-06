use rocket::tokio::time::{sleep, Duration};
use rocket::{get, serde::json::Json};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
struct Message {
    message: String,
}

/// Hello World!
///
/// Index page that returns "Hello World"
#[openapi(tag = "Message")]
#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        message: "Hello World!".to_owned(),
    })
}

/// Delay by Seconds
///
/// Wait x seconds until respondsing
#[openapi(tag = "Message")]
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> Json<Message> {
    sleep(Duration::from_secs(seconds)).await;

    Json(Message {
        message: format!("Waited for {} seconds", seconds),
    })
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![index, delay])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
