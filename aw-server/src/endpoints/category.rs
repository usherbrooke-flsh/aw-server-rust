use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::{JsonValue};

use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;

use crate::endpoints::{HttpErrorJson, ServerState};

#[get("/")]
pub fn categories_get(_state: State<ServerState>) -> Result<JsonValue, HttpErrorJson> {
    let client = reqwest::blocking::Client::new();
    let res_categories = match client.get("https://espaceun.uqam.ca/rest-v1/xrxh_mrps/")
        .header(AUTHORIZATION, "Basic ZG91YmxlZGFzaGF3c2VjcmV0aWQ=")
        .header(CONTENT_TYPE, "application/json")
        .send() {
            Ok(data) => data,
            Err(e) => {
                warn!("Category query failed: {:?}", e);
                return Err(HttpErrorJson::new(
                    Status::InternalServerError,
                    e.to_string(),
                ));
            }
        };

    let categories = match res_categories.json::<serde_json::Value>() {
        Ok(data) => data,
        Err(e) => {
            warn!("Category encoding failed: {:?}", e);
            return Err(HttpErrorJson::new(
                Status::InternalServerError,
                e.to_string(),
            ));
        }
    };

    let res_sub = match client.get("https://espaceun.uqam.ca/rest-v1/xrxh_eaed/")
        .header(AUTHORIZATION, "Basic ZG91YmxlZGFzaGF3c2VjcmV0aWQ=")
        .header(CONTENT_TYPE, "application/json")
        .send() {
            Ok(data) => data,
            Err(e) => {
                warn!("Sub-category query failed: {:?}", e);
                return Err(HttpErrorJson::new(
                    Status::InternalServerError,
                    e.to_string(),
                ));
            }
        };

    let sub_categories = match res_sub.json::<serde_json::Value>() {
        Ok(data) => data,
        Err(e) => {
            warn!("Sub-category encoding failed: {:?}", e);
            return Err(HttpErrorJson::new(
                Status::InternalServerError,
                e.to_string(),
            ));
        }
    };

    Ok(json!({"categories": categories, "sub": sub_categories}))
}
