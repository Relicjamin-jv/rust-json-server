use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};
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

async fn create_kube_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    Ok(client)
}

async fn list_all_pods() -> Result<Json<Message>, Box<dyn std::error::Error>> {
    let client = create_kube_client().await.unwrap();

    let mut message: String = "".to_owned();
    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&ListParams::default()).await? {
        message.push_str(&*p.name_any())
    }

    Ok(Json(Message { message }))
}

#[openapi(tag = "Message")]
#[get("/pods")]
async fn get_pods() -> Json<Message> {
    list_all_pods().await.unwrap()
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![index, delay, get_pods])
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
