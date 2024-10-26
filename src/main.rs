use rocket::{State, get, launch, routes};
use rocket::serde::{Deserialize, Serialize};// From Rocket’s custom serde integration
use rocket::http::Status;
use serde_json::json;
use rocket::fairing::AdHoc;
use sqlx::prelude::FromRow;
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use dotenvy::dotenv;
use std::{env, fs};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Pong {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Location {
    coordinates: serde_json::Value,
    city: serde_json::Value,
    country: serde_json::Value,
}

#[get("/ping")]
fn ping() -> serde_json::Value {
    json!(Pong { message: "Pong!".to_string() })
}

#[get("/location")]
async fn location() -> Result<serde_json::Value, ()> {
    let ip_response: String = reqwest::get("https://ifconfig.me")
        .await.unwrap()
        .text()
        .await.unwrap();

    // Step 1: Use the IP address to fetch geolocation data
    let geo_response: serde_json::Value = reqwest::get(format!("https://ipinfo.io/{}/json", ip_response.trim()))
        .await.unwrap()
        .json()
        .await.unwrap();

    Ok(json!( Location {
        coordinates: geo_response["loc"].clone(),
        city: geo_response["city"].clone(),
        country: geo_response["country"].clone()
    }))
}

#[derive(Serialize, Deserialize, FromRow, Default)]
struct Snapshot {
    id: i64,
    date: String,
    battery: i64,
    solar_panel_wattage: Option<f64>,
    city: String
}

async fn get_latest_snapshot(pool: &SqlitePool) -> Result<Snapshot, sqlx::Error> {
    let snapshot = sqlx::query_as!(
        Snapshot,
        "
        SELECT * FROM snapshot
        ORDER BY id DESC
        LIMIT 1;
        "
    )
    .fetch_one(pool)
    .await?;

    // Ok(snapshot)
    Ok(snapshot)
}

#[get("/snapshot")]
async fn snapshot(db: &State<SqlitePool>) -> Result<serde_json::Value, Status> {
    match get_latest_snapshot(db).await {
        Ok(snapshot) => Ok(json!(snapshot)),
        Err(_) => Err(Status::new(500)),
    }
}

async fn init_db() -> SqlitePool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect_with(SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(&database_url.replace("sqlite://", ""))
        )  
        .await
        .expect("Failed to create pool");
    
    run_migrations(&pool).await.expect("Failed to run migrations");

    pool
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let migration_path = Path::new("./migrations");

    for entry in fs::read_dir(migration_path).expect("Failed to read migration directory") {
        let path = entry.expect("Failed to read migration entry").path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("sql") {
            let sql = fs::read_to_string(path).expect("Failed to read migration file");
            sqlx::query(&sql).execute(pool).await?;
        }
    }

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Database Setup", |rocket| async {
            let pool = init_db().await;
            rocket.manage(pool)
        }))
        .mount("/", routes![ping, location, snapshot])
}
