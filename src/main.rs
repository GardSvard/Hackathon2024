use rocket::{get, launch, routes};
use rocket::serde::{Deserialize, Serialize};// From Rocket’s custom serde integration
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct PongResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct LocationResponse {
    coordinates: serde_json::Value,
    city: serde_json::Value,
    country: serde_json::Value,
}

#[get("/ping")]
fn ping() -> serde_json::Value {
    json!(PongResponse { message: "Pong!".to_string() })
}

#[get("/location")]
async fn location() -> Result<serde_json::Value, ()> {
    let ip_response: String = reqwest::get("https://ifconfig.me")
        .await.unwrap()
        .text()
        .await.unwrap();

    // Step 2: Use the IP address to fetch geolocation data
    let geo_response: serde_json::Value = reqwest::get(format!("https://ipinfo.io/{}/json", ip_response.trim()))
        .await.unwrap()
        .json()
        .await.unwrap();

    Ok(json!( LocationResponse {
        coordinates: geo_response["loc"].clone(),
        city: geo_response["city"].clone(),
        country: geo_response["country"].clone()
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping, location])
}
