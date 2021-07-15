use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{JsonValue};

use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

use crate::endpoints::{HttpErrorJson, ServerState};

#[get("/")]
pub fn translations_get(_state: State<ServerState>) -> Result<JsonValue, HttpErrorJson> {
    let client = reqwest::blocking::Client::new();
    let response = match client.get("https://espaceun.uqam.ca/rest-v1/xrxh_imsw/")
        .header(AUTHORIZATION, "Basic ZG91YmxlZGFzaGF3c2VjcmV0aWQ=")
        .header(CONTENT_TYPE, "application/json")
        .send() {
            Ok(data) => data,
            Err(e) => {
                warn!("Query failed: {:?}", e);
                return Err(HttpErrorJson::new(
                    Status::InternalServerError,
                    e.to_string(),
                ));
            }
        };

    let translations = match response.json::<serde_json::Value>() {
        Ok(data) => data,
        Err(e) => {
            warn!("Query failed: {:?}", e);
            return Err(HttpErrorJson::new(
                Status::InternalServerError,
                e.to_string(),
            ));
        }
    };

    Ok(json!(translations))
}
